// @generated
impl serde::Serialize for ActivateWebKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.instance.is_some() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.webkey.v3alpha.ActivateWebKeyRequest", len)?;
        if let Some(v) = self.instance.as_ref() {
            struct_ser.serialize_field("instance", v)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActivateWebKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance",
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Instance,
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "instance" => Ok(GeneratedField::Instance),
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActivateWebKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.webkey.v3alpha.ActivateWebKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActivateWebKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance__ = None;
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Instance => {
                            if instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            instance__ = map_.next_value()?;
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ActivateWebKeyRequest {
                    instance: instance__,
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.webkey.v3alpha.ActivateWebKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ActivateWebKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.webkey.v3alpha.ActivateWebKeyResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActivateWebKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "details" => Ok(GeneratedField::Details),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActivateWebKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.webkey.v3alpha.ActivateWebKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActivateWebKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ActivateWebKeyResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.webkey.v3alpha.ActivateWebKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWebKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.instance.is_some() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.webkey.v3alpha.CreateWebKeyRequest", len)?;
        if let Some(v) = self.instance.as_ref() {
            struct_ser.serialize_field("instance", v)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWebKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Instance,
            Key,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "instance" => Ok(GeneratedField::Instance),
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateWebKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.webkey.v3alpha.CreateWebKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateWebKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance__ = None;
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Instance => {
                            if instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            instance__ = map_.next_value()?;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateWebKeyRequest {
                    instance: instance__,
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.webkey.v3alpha.CreateWebKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWebKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.webkey.v3alpha.CreateWebKeyResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWebKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "details" => Ok(GeneratedField::Details),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateWebKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.webkey.v3alpha.CreateWebKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateWebKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateWebKeyResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.webkey.v3alpha.CreateWebKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteWebKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.instance.is_some() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.webkey.v3alpha.DeleteWebKeyRequest", len)?;
        if let Some(v) = self.instance.as_ref() {
            struct_ser.serialize_field("instance", v)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteWebKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance",
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Instance,
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "instance" => Ok(GeneratedField::Instance),
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteWebKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.webkey.v3alpha.DeleteWebKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteWebKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance__ = None;
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Instance => {
                            if instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            instance__ = map_.next_value()?;
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteWebKeyRequest {
                    instance: instance__,
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.webkey.v3alpha.DeleteWebKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteWebKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.webkey.v3alpha.DeleteWebKeyResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteWebKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "details" => Ok(GeneratedField::Details),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteWebKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.webkey.v3alpha.DeleteWebKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteWebKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeleteWebKeyResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.webkey.v3alpha.DeleteWebKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWebKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.details.is_some() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.webkey.v3alpha.GetWebKey", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if self.state != 0 {
            let v = WebKeyState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWebKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "config",
            "state",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Config,
            State,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "details" => Ok(GeneratedField::Details),
                            "config" => Ok(GeneratedField::Config),
                            "state" => Ok(GeneratedField::State),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWebKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.webkey.v3alpha.GetWebKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWebKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut config__ = None;
                let mut state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<WebKeyState>()? as i32);
                        }
                    }
                }
                Ok(GetWebKey {
                    details: details__,
                    config: config__,
                    state: state__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.webkey.v3alpha.GetWebKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWebKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.instance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.webkey.v3alpha.ListWebKeysRequest", len)?;
        if let Some(v) = self.instance.as_ref() {
            struct_ser.serialize_field("instance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWebKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Instance,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "instance" => Ok(GeneratedField::Instance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListWebKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.webkey.v3alpha.ListWebKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWebKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Instance => {
                            if instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            instance__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListWebKeysRequest {
                    instance: instance__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.webkey.v3alpha.ListWebKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWebKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.web_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.webkey.v3alpha.ListWebKeysResponse", len)?;
        if !self.web_keys.is_empty() {
            struct_ser.serialize_field("webKeys", &self.web_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWebKeysResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "web_keys",
            "webKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebKeys,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "webKeys" | "web_keys" => Ok(GeneratedField::WebKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListWebKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.webkey.v3alpha.ListWebKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWebKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut web_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebKeys => {
                            if web_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webKeys"));
                            }
                            web_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListWebKeysResponse {
                    web_keys: web_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.webkey.v3alpha.ListWebKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.webkey.v3alpha.WebKey", len)?;
        if let Some(v) = self.config.as_ref() {
            match v {
                web_key::Config::Rsa(v) => {
                    struct_ser.serialize_field("rsa", v)?;
                }
                web_key::Config::Ecdsa(v) => {
                    struct_ser.serialize_field("ecdsa", v)?;
                }
                web_key::Config::Ed25519(v) => {
                    struct_ser.serialize_field("ed25519", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rsa",
            "ecdsa",
            "ed25519",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rsa,
            Ecdsa,
            Ed25519,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rsa" => Ok(GeneratedField::Rsa),
                            "ecdsa" => Ok(GeneratedField::Ecdsa),
                            "ed25519" => Ok(GeneratedField::Ed25519),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.webkey.v3alpha.WebKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rsa => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rsa"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(web_key::Config::Rsa)
;
                        }
                        GeneratedField::Ecdsa => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ecdsa"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(web_key::Config::Ecdsa)
;
                        }
                        GeneratedField::Ed25519 => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ed25519"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(web_key::Config::Ed25519)
;
                        }
                    }
                }
                Ok(WebKey {
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.webkey.v3alpha.WebKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebKeyEcdsaConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.curve != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.webkey.v3alpha.WebKeyECDSAConfig", len)?;
        if self.curve != 0 {
            let v = web_key_ecdsa_config::EcdsaCurve::try_from(self.curve)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.curve)))?;
            struct_ser.serialize_field("curve", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebKeyEcdsaConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "curve",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Curve,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "curve" => Ok(GeneratedField::Curve),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebKeyEcdsaConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.webkey.v3alpha.WebKeyECDSAConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebKeyEcdsaConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut curve__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Curve => {
                            if curve__.is_some() {
                                return Err(serde::de::Error::duplicate_field("curve"));
                            }
                            curve__ = Some(map_.next_value::<web_key_ecdsa_config::EcdsaCurve>()? as i32);
                        }
                    }
                }
                Ok(WebKeyEcdsaConfig {
                    curve: curve__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.webkey.v3alpha.WebKeyECDSAConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for web_key_ecdsa_config::EcdsaCurve {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ECDSA_CURVE_UNSPECIFIED",
            Self::P256 => "ECDSA_CURVE_P256",
            Self::P384 => "ECDSA_CURVE_P384",
            Self::P512 => "ECDSA_CURVE_P512",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for web_key_ecdsa_config::EcdsaCurve {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ECDSA_CURVE_UNSPECIFIED",
            "ECDSA_CURVE_P256",
            "ECDSA_CURVE_P384",
            "ECDSA_CURVE_P512",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = web_key_ecdsa_config::EcdsaCurve;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ECDSA_CURVE_UNSPECIFIED" => Ok(web_key_ecdsa_config::EcdsaCurve::Unspecified),
                    "ECDSA_CURVE_P256" => Ok(web_key_ecdsa_config::EcdsaCurve::P256),
                    "ECDSA_CURVE_P384" => Ok(web_key_ecdsa_config::EcdsaCurve::P384),
                    "ECDSA_CURVE_P512" => Ok(web_key_ecdsa_config::EcdsaCurve::P512),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WebKeyEd25519Config {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.resources.webkey.v3alpha.WebKeyED25519Config", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebKeyEd25519Config {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebKeyEd25519Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.webkey.v3alpha.WebKeyED25519Config")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebKeyEd25519Config, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(WebKeyEd25519Config {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.webkey.v3alpha.WebKeyED25519Config", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebKeyRsaConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bits != 0 {
            len += 1;
        }
        if self.hasher != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.webkey.v3alpha.WebKeyRSAConfig", len)?;
        if self.bits != 0 {
            let v = web_key_rsa_config::RsaBits::try_from(self.bits)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.bits)))?;
            struct_ser.serialize_field("bits", &v)?;
        }
        if self.hasher != 0 {
            let v = web_key_rsa_config::RsaHasher::try_from(self.hasher)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.hasher)))?;
            struct_ser.serialize_field("hasher", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebKeyRsaConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bits",
            "hasher",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bits,
            Hasher,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bits" => Ok(GeneratedField::Bits),
                            "hasher" => Ok(GeneratedField::Hasher),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebKeyRsaConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.webkey.v3alpha.WebKeyRSAConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebKeyRsaConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bits__ = None;
                let mut hasher__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bits => {
                            if bits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bits"));
                            }
                            bits__ = Some(map_.next_value::<web_key_rsa_config::RsaBits>()? as i32);
                        }
                        GeneratedField::Hasher => {
                            if hasher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasher"));
                            }
                            hasher__ = Some(map_.next_value::<web_key_rsa_config::RsaHasher>()? as i32);
                        }
                    }
                }
                Ok(WebKeyRsaConfig {
                    bits: bits__.unwrap_or_default(),
                    hasher: hasher__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.webkey.v3alpha.WebKeyRSAConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for web_key_rsa_config::RsaBits {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "RSA_BITS_UNSPECIFIED",
            Self::RsaBits2048 => "RSA_BITS_2048",
            Self::RsaBits3072 => "RSA_BITS_3072",
            Self::RsaBits4096 => "RSA_BITS_4096",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for web_key_rsa_config::RsaBits {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RSA_BITS_UNSPECIFIED",
            "RSA_BITS_2048",
            "RSA_BITS_3072",
            "RSA_BITS_4096",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = web_key_rsa_config::RsaBits;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "RSA_BITS_UNSPECIFIED" => Ok(web_key_rsa_config::RsaBits::Unspecified),
                    "RSA_BITS_2048" => Ok(web_key_rsa_config::RsaBits::RsaBits2048),
                    "RSA_BITS_3072" => Ok(web_key_rsa_config::RsaBits::RsaBits3072),
                    "RSA_BITS_4096" => Ok(web_key_rsa_config::RsaBits::RsaBits4096),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for web_key_rsa_config::RsaHasher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "RSA_HASHER_UNSPECIFIED",
            Self::Sha256 => "RSA_HASHER_SHA256",
            Self::Sha384 => "RSA_HASHER_SHA384",
            Self::Sha512 => "RSA_HASHER_SHA512",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for web_key_rsa_config::RsaHasher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RSA_HASHER_UNSPECIFIED",
            "RSA_HASHER_SHA256",
            "RSA_HASHER_SHA384",
            "RSA_HASHER_SHA512",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = web_key_rsa_config::RsaHasher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "RSA_HASHER_UNSPECIFIED" => Ok(web_key_rsa_config::RsaHasher::Unspecified),
                    "RSA_HASHER_SHA256" => Ok(web_key_rsa_config::RsaHasher::Sha256),
                    "RSA_HASHER_SHA384" => Ok(web_key_rsa_config::RsaHasher::Sha384),
                    "RSA_HASHER_SHA512" => Ok(web_key_rsa_config::RsaHasher::Sha512),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WebKeyState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::StateUnspecified => "STATE_UNSPECIFIED",
            Self::StateInitial => "STATE_INITIAL",
            Self::StateActive => "STATE_ACTIVE",
            Self::StateInactive => "STATE_INACTIVE",
            Self::StateRemoved => "STATE_REMOVED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for WebKeyState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATE_UNSPECIFIED",
            "STATE_INITIAL",
            "STATE_ACTIVE",
            "STATE_INACTIVE",
            "STATE_REMOVED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebKeyState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STATE_UNSPECIFIED" => Ok(WebKeyState::StateUnspecified),
                    "STATE_INITIAL" => Ok(WebKeyState::StateInitial),
                    "STATE_ACTIVE" => Ok(WebKeyState::StateActive),
                    "STATE_INACTIVE" => Ok(WebKeyState::StateInactive),
                    "STATE_REMOVED" => Ok(WebKeyState::StateRemoved),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
