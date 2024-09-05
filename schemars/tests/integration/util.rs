use core::f64;
use jsonschema::JSONSchema as CompiledSchema;
use schemars::{
    generate::{Contract, SchemaSettings},
    JsonSchema, Schema,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use snapbox::IntoJson;
use std::{cell::OnceCell, marker::PhantomData, path::Path, sync::OnceLock};

pub struct Test<T: JsonSchema> {
    settings: SchemaSettings,
    name: &'static str,
    phantom: PhantomData<T>,
    de_schema: OnceCell<Schema>,
    ser_schema: OnceCell<Schema>,
    de_schema_compiled: OnceCell<CompiledSchema>,
    ser_schema_compiled: OnceCell<CompiledSchema>,
}

impl<T: JsonSchema> Test<T> {
    pub fn new(name: &'static str, settings: SchemaSettings) -> Self {
        Self {
            settings,
            name,
            phantom: PhantomData,
            de_schema: OnceCell::new(),
            ser_schema: OnceCell::new(),
            de_schema_compiled: OnceCell::new(),
            ser_schema_compiled: OnceCell::new(),
        }
    }

    pub fn assert_snapshot(&self) -> &Self {
        let name = self
            .name
            .replace("::", "_")
            .replace('<', "_")
            .replace(',', "-")
            .replace(|c: char| c != '_' && c != '-' && !c.is_alphanumeric(), "");

        let de_path = format!("tests/integration/snapshots/{name}.de.json");
        snapbox::assert_data_eq!(
            self.de_schema().into_json(),
            snapbox::Data::read_from(Path::new(&de_path), None)
        );

        let ser_path = format!("tests/integration/snapshots/{name}.ser.json");
        snapbox::assert_data_eq!(
            self.ser_schema().into_json(),
            snapbox::Data::read_from(Path::new(&ser_path), None)
        );

        self
    }

    pub fn assert_identical<T2: JsonSchema>(&self) -> &Self {
        snapbox::assert_data_eq!(
            self.de_schema().into_json(),
            self.schema_for::<T2>(Contract::Deserialize).into_json()
        );
        snapbox::assert_data_eq!(
            self.ser_schema().into_json(),
            self.schema_for::<T2>(Contract::Serialize).into_json()
        );
        self
    }

    fn schema_for<T2: JsonSchema>(&self, contract: Contract) -> Schema {
        self.settings
            .clone()
            .with(|s| s.contract = contract)
            .into_generator()
            .into_root_schema_for::<T2>()
    }

    fn de_schema(&self) -> &Schema {
        self.de_schema
            .get_or_init(|| self.schema_for::<T>(Contract::Deserialize))
    }

    fn ser_schema(&self) -> &Schema {
        self.ser_schema
            .get_or_init(|| self.schema_for::<T>(Contract::Serialize))
    }

    fn de_schema_compiled(&self) -> &CompiledSchema {
        self.de_schema_compiled.get_or_init(|| {
            CompiledSchema::compile(self.de_schema().as_value()).expect("valid deserialize schema")
        })
    }
    fn ser_schema_compiled(&self) -> &CompiledSchema {
        self.ser_schema_compiled.get_or_init(|| {
            CompiledSchema::compile(self.ser_schema().as_value()).expect("valid serialize schema")
        })
    }
}

impl<T: JsonSchema + Serialize + for<'de> Deserialize<'de>> Test<T> {
    pub fn assert_allows_serde_roundtrip(&self, samples: impl IntoIterator<Item = T>) -> &Self {
        let ser_schema = self.ser_schema_compiled();
        let de_schema = self.de_schema_compiled();

        for sample in samples {
            let json = serde_json::to_value(sample).unwrap();
            assert!(
                ser_schema.is_valid(&json),
                "serialize schema should allow serialized value: {json}"
            );

            assert!(
                T::deserialize(&json).is_ok(),
                "sanity check - ser/de roundtrip: {json}"
            );

            assert!(
                de_schema.is_valid(&json),
                "deserialize schema should allow value accepted by deserialization: {json}"
            );
        }

        self
    }
}

impl<T: JsonSchema + Serialize + for<'de> Deserialize<'de> + Default> Test<T> {
    pub fn assert_allows_serde_roundtrip_default(&self) -> &Self {
        self.assert_allows_serde_roundtrip([T::default()])
    }
}

impl<T: JsonSchema + for<'de> Deserialize<'de>> Test<T> {
    pub fn assert_rejects(&self, values: impl IntoIterator<Item = Value>) -> &Self {
        let ser_schema = self.ser_schema_compiled();
        let de_schema = self.de_schema_compiled();

        for value in values {
            assert!(
                !ser_schema.is_valid(&value),
                "serialize schema should reject invalid value: {value}"
            );

            assert!(
                T::deserialize(&value).is_err(),
                "sanity check - invalid value should fail deserialization: {value}"
            );

            assert!(
                !de_schema.is_valid(&value),
                "deserialize schema should reject invalid value: {value}"
            );
        }

        self
    }

    pub fn assert_matches_deserialize<'a>(&self, values: impl Iterator<Item = &'a Value>) -> &Self {
        let ser_schema = self.ser_schema_compiled();
        let de_schema = self.de_schema_compiled();

        for value in values {
            if T::deserialize(value).is_ok() {
                assert!(
                    de_schema.is_valid(value),
                    "deserialize schema should allow value accepted by deserialization: {value}"
                );

                // Don't check ser_schema because it may be stricter than de_schema

                continue;
            }

            assert!(
                !de_schema.is_valid(value),
                "deserialize schema should reject invalid value: {value}"
            );

            assert!(
                !ser_schema.is_valid(value),
                "serialize schema should reject invalid value: {value}"
            );
        }

        self
    }
}

pub fn arbitrary_values() -> impl Iterator<Item = &'static Value> {
    static VALUES: OnceLock<Vec<Value>> = OnceLock::new();
    VALUES
        .get_or_init(|| {
            vec![
                Value::Null,
                false.into(),
                true.into(),
                0.into(),
                255.into(),
                (-1).into(),
                u64::MAX.into(),
                f64::consts::PI.into(),
                "".into(),
                "0".into(),
                "3E8".into(),
                "Pâté costs £1\r\n\0".into(),
                Value::Array(Default::default()),
                Value::Object(Default::default()),
            ]
        })
        .iter()
}

pub fn arbitrary_nonstring_values() -> impl Iterator<Item = &'static Value> {
    arbitrary_values().filter(|v| !v.is_string())
}
