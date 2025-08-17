// @generated
impl serde::Serialize for Condition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.condition_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.Condition", len)?;
        if let Some(v) = self.condition_type.as_ref() {
            match v {
                condition::ConditionType::Request(v) => {
                    struct_ser.serialize_field("request", v)?;
                }
                condition::ConditionType::Response(v) => {
                    struct_ser.serialize_field("response", v)?;
                }
                condition::ConditionType::Function(v) => {
                    struct_ser.serialize_field("function", v)?;
                }
                condition::ConditionType::Event(v) => {
                    struct_ser.serialize_field("event", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Condition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request",
            "response",
            "function",
            "event",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Request,
            Response,
            Function,
            Event,
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
                            "request" => Ok(GeneratedField::Request),
                            "response" => Ok(GeneratedField::Response),
                            "function" => Ok(GeneratedField::Function),
                            "event" => Ok(GeneratedField::Event),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Condition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.Condition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Condition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut condition_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Request => {
                            if condition_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            condition_type__ = map_.next_value::<::std::option::Option<_>>()?.map(condition::ConditionType::Request)
;
                        }
                        GeneratedField::Response => {
                            if condition_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            condition_type__ = map_.next_value::<::std::option::Option<_>>()?.map(condition::ConditionType::Response)
;
                        }
                        GeneratedField::Function => {
                            if condition_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            condition_type__ = map_.next_value::<::std::option::Option<_>>()?.map(condition::ConditionType::Function)
;
                        }
                        GeneratedField::Event => {
                            if condition_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("event"));
                            }
                            condition_type__ = map_.next_value::<::std::option::Option<_>>()?.map(condition::ConditionType::Event)
;
                        }
                    }
                }
                Ok(Condition {
                    condition_type: condition_type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.Condition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTargetRequest {
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
        if self.target.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.CreateTargetRequest", len)?;
        if let Some(v) = self.instance.as_ref() {
            struct_ser.serialize_field("instance", v)?;
        }
        if let Some(v) = self.target.as_ref() {
            struct_ser.serialize_field("target", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTargetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance",
            "target",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Instance,
            Target,
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
                            "target" => Ok(GeneratedField::Target),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTargetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.CreateTargetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTargetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance__ = None;
                let mut target__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Instance => {
                            if instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            instance__ = map_.next_value()?;
                        }
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateTargetRequest {
                    instance: instance__,
                    target: target__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.CreateTargetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTargetResponse {
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
        if !self.signing_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.CreateTargetResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.signing_key.is_empty() {
            struct_ser.serialize_field("signingKey", &self.signing_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTargetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "signing_key",
            "signingKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            SigningKey,
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
                            "signingKey" | "signing_key" => Ok(GeneratedField::SigningKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTargetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.CreateTargetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTargetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut signing_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::SigningKey => {
                            if signing_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signingKey"));
                            }
                            signing_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateTargetResponse {
                    details: details__,
                    signing_key: signing_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.CreateTargetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTargetRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.DeleteTargetRequest", len)?;
        if let Some(v) = self.instance.as_ref() {
            struct_ser.serialize_field("instance", v)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTargetRequest {
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
            type Value = DeleteTargetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.DeleteTargetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTargetRequest, V::Error>
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
                Ok(DeleteTargetRequest {
                    instance: instance__,
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.DeleteTargetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTargetResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.DeleteTargetResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTargetResponse {
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
            type Value = DeleteTargetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.DeleteTargetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTargetResponse, V::Error>
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
                Ok(DeleteTargetResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.DeleteTargetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventExecution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.condition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.EventExecution", len)?;
        if let Some(v) = self.condition.as_ref() {
            match v {
                event_execution::Condition::Event(v) => {
                    struct_ser.serialize_field("event", v)?;
                }
                event_execution::Condition::Group(v) => {
                    struct_ser.serialize_field("group", v)?;
                }
                event_execution::Condition::All(v) => {
                    struct_ser.serialize_field("all", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventExecution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event",
            "group",
            "all",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Event,
            Group,
            All,
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
                            "event" => Ok(GeneratedField::Event),
                            "group" => Ok(GeneratedField::Group),
                            "all" => Ok(GeneratedField::All),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventExecution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.EventExecution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventExecution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut condition__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Event => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("event"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(event_execution::Condition::Event);
                        }
                        GeneratedField::Group => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(event_execution::Condition::Group);
                        }
                        GeneratedField::All => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("all"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(event_execution::Condition::All);
                        }
                    }
                }
                Ok(EventExecution {
                    condition: condition__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.EventExecution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Execution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.targets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.Execution", len)?;
        if !self.targets.is_empty() {
            struct_ser.serialize_field("targets", &self.targets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Execution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "targets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Targets,
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
                            "targets" => Ok(GeneratedField::Targets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Execution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.Execution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Execution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut targets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Targets => {
                            if targets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targets"));
                            }
                            targets__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Execution {
                    targets: targets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.Execution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecutionFieldName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "EXECUTION_FIELD_NAME_UNSPECIFIED",
            Self::Id => "EXECUTION_FIELD_NAME_ID",
            Self::CreatedDate => "EXECUTION_FIELD_NAME_CREATED_DATE",
            Self::ChangedDate => "EXECUTION_FIELD_NAME_CHANGED_DATE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ExecutionFieldName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EXECUTION_FIELD_NAME_UNSPECIFIED",
            "EXECUTION_FIELD_NAME_ID",
            "EXECUTION_FIELD_NAME_CREATED_DATE",
            "EXECUTION_FIELD_NAME_CHANGED_DATE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecutionFieldName;

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
                    "EXECUTION_FIELD_NAME_UNSPECIFIED" => Ok(ExecutionFieldName::Unspecified),
                    "EXECUTION_FIELD_NAME_ID" => Ok(ExecutionFieldName::Id),
                    "EXECUTION_FIELD_NAME_CREATED_DATE" => Ok(ExecutionFieldName::CreatedDate),
                    "EXECUTION_FIELD_NAME_CHANGED_DATE" => Ok(ExecutionFieldName::ChangedDate),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ExecutionSearchFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.ExecutionSearchFilter", len)?;
        if let Some(v) = self.filter.as_ref() {
            match v {
                execution_search_filter::Filter::InConditionsFilter(v) => {
                    struct_ser.serialize_field("inConditionsFilter", v)?;
                }
                execution_search_filter::Filter::ExecutionTypeFilter(v) => {
                    struct_ser.serialize_field("executionTypeFilter", v)?;
                }
                execution_search_filter::Filter::TargetFilter(v) => {
                    struct_ser.serialize_field("targetFilter", v)?;
                }
                execution_search_filter::Filter::IncludeFilter(v) => {
                    struct_ser.serialize_field("includeFilter", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecutionSearchFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "in_conditions_filter",
            "inConditionsFilter",
            "execution_type_filter",
            "executionTypeFilter",
            "target_filter",
            "targetFilter",
            "include_filter",
            "includeFilter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InConditionsFilter,
            ExecutionTypeFilter,
            TargetFilter,
            IncludeFilter,
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
                            "inConditionsFilter" | "in_conditions_filter" => Ok(GeneratedField::InConditionsFilter),
                            "executionTypeFilter" | "execution_type_filter" => Ok(GeneratedField::ExecutionTypeFilter),
                            "targetFilter" | "target_filter" => Ok(GeneratedField::TargetFilter),
                            "includeFilter" | "include_filter" => Ok(GeneratedField::IncludeFilter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecutionSearchFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.ExecutionSearchFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecutionSearchFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InConditionsFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inConditionsFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(execution_search_filter::Filter::InConditionsFilter)
;
                        }
                        GeneratedField::ExecutionTypeFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionTypeFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(execution_search_filter::Filter::ExecutionTypeFilter)
;
                        }
                        GeneratedField::TargetFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(execution_search_filter::Filter::TargetFilter)
;
                        }
                        GeneratedField::IncludeFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(execution_search_filter::Filter::IncludeFilter)
;
                        }
                    }
                }
                Ok(ExecutionSearchFilter {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.ExecutionSearchFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecutionTargetType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.ExecutionTargetType", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                execution_target_type::Type::Target(v) => {
                    struct_ser.serialize_field("target", v)?;
                }
                execution_target_type::Type::Include(v) => {
                    struct_ser.serialize_field("include", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecutionTargetType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target",
            "include",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Target,
            Include,
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
                            "target" => Ok(GeneratedField::Target),
                            "include" => Ok(GeneratedField::Include),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecutionTargetType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.ExecutionTargetType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecutionTargetType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Target => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(execution_target_type::Type::Target);
                        }
                        GeneratedField::Include => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("include"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(execution_target_type::Type::Include)
;
                        }
                    }
                }
                Ok(ExecutionTargetType {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.ExecutionTargetType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecutionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "EXECUTION_TYPE_UNSPECIFIED",
            Self::Request => "EXECUTION_TYPE_REQUEST",
            Self::Response => "EXECUTION_TYPE_RESPONSE",
            Self::Event => "EXECUTION_TYPE_EVENT",
            Self::Function => "EXECUTION_TYPE_FUNCTION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ExecutionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EXECUTION_TYPE_UNSPECIFIED",
            "EXECUTION_TYPE_REQUEST",
            "EXECUTION_TYPE_RESPONSE",
            "EXECUTION_TYPE_EVENT",
            "EXECUTION_TYPE_FUNCTION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecutionType;

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
                    "EXECUTION_TYPE_UNSPECIFIED" => Ok(ExecutionType::Unspecified),
                    "EXECUTION_TYPE_REQUEST" => Ok(ExecutionType::Request),
                    "EXECUTION_TYPE_RESPONSE" => Ok(ExecutionType::Response),
                    "EXECUTION_TYPE_EVENT" => Ok(ExecutionType::Event),
                    "EXECUTION_TYPE_FUNCTION" => Ok(ExecutionType::Function),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ExecutionTypeFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.execution_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.ExecutionTypeFilter", len)?;
        if self.execution_type != 0 {
            let v = ExecutionType::try_from(self.execution_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.execution_type)))?;
            struct_ser.serialize_field("executionType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecutionTypeFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "execution_type",
            "executionType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExecutionType,
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
                            "executionType" | "execution_type" => Ok(GeneratedField::ExecutionType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecutionTypeFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.ExecutionTypeFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecutionTypeFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut execution_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExecutionType => {
                            if execution_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionType"));
                            }
                            execution_type__ = Some(map_.next_value::<ExecutionType>()? as i32);
                        }
                    }
                }
                Ok(ExecutionTypeFilter {
                    execution_type: execution_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.ExecutionTypeFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FunctionExecution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.FunctionExecution", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FunctionExecution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FunctionExecution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.FunctionExecution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FunctionExecution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FunctionExecution {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.FunctionExecution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetExecution {
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
        if self.condition.is_some() {
            len += 1;
        }
        if self.execution.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.GetExecution", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.condition.as_ref() {
            struct_ser.serialize_field("condition", v)?;
        }
        if let Some(v) = self.execution.as_ref() {
            struct_ser.serialize_field("execution", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetExecution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "condition",
            "execution",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Condition,
            Execution,
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
                            "condition" => Ok(GeneratedField::Condition),
                            "execution" => Ok(GeneratedField::Execution),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetExecution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.GetExecution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetExecution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut condition__ = None;
                let mut execution__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Condition => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            condition__ = map_.next_value()?;
                        }
                        GeneratedField::Execution => {
                            if execution__.is_some() {
                                return Err(serde::de::Error::duplicate_field("execution"));
                            }
                            execution__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetExecution {
                    details: details__,
                    condition: condition__,
                    execution: execution__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.GetExecution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTarget {
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
        if !self.signing_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.GetTarget", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if !self.signing_key.is_empty() {
            struct_ser.serialize_field("signingKey", &self.signing_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTarget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "config",
            "signing_key",
            "signingKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Config,
            SigningKey,
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
                            "signingKey" | "signing_key" => Ok(GeneratedField::SigningKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTarget;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.GetTarget")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTarget, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut config__ = None;
                let mut signing_key__ = None;
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
                        GeneratedField::SigningKey => {
                            if signing_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signingKey"));
                            }
                            signing_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetTarget {
                    details: details__,
                    config: config__,
                    signing_key: signing_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.GetTarget", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTargetRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.GetTargetRequest", len)?;
        if let Some(v) = self.instance.as_ref() {
            struct_ser.serialize_field("instance", v)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTargetRequest {
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
            type Value = GetTargetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.GetTargetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTargetRequest, V::Error>
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
                Ok(GetTargetRequest {
                    instance: instance__,
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.GetTargetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTargetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.target.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.GetTargetResponse", len)?;
        if let Some(v) = self.target.as_ref() {
            struct_ser.serialize_field("target", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTargetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Target,
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
                            "target" => Ok(GeneratedField::Target),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTargetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.GetTargetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTargetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetTargetResponse {
                    target: target__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.GetTargetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InConditionsFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.conditions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.InConditionsFilter", len)?;
        if !self.conditions.is_empty() {
            struct_ser.serialize_field("conditions", &self.conditions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InConditionsFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conditions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Conditions,
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
                            "conditions" => Ok(GeneratedField::Conditions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InConditionsFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.InConditionsFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InConditionsFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conditions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Conditions => {
                            if conditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditions"));
                            }
                            conditions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InConditionsFilter {
                    conditions: conditions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.InConditionsFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InTargetIDsFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.target_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.InTargetIDsFilter", len)?;
        if !self.target_ids.is_empty() {
            struct_ser.serialize_field("targetIds", &self.target_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InTargetIDsFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target_ids",
            "targetIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TargetIds,
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
                            "targetIds" | "target_ids" => Ok(GeneratedField::TargetIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InTargetIDsFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.InTargetIDsFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InTargetIDsFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TargetIds => {
                            if target_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetIds"));
                            }
                            target_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InTargetIDsFilter {
                    target_ids: target_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.InTargetIDsFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IncludeFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.include.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.IncludeFilter", len)?;
        if let Some(v) = self.include.as_ref() {
            struct_ser.serialize_field("include", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IncludeFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "include",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Include,
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
                            "include" => Ok(GeneratedField::Include),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IncludeFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.IncludeFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IncludeFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut include__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Include => {
                            if include__.is_some() {
                                return Err(serde::de::Error::duplicate_field("include"));
                            }
                            include__ = map_.next_value()?;
                        }
                    }
                }
                Ok(IncludeFilter {
                    include: include__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.IncludeFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExecutionFunctionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.ListExecutionFunctionsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExecutionFunctionsRequest {
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
            type Value = ListExecutionFunctionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.ListExecutionFunctionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExecutionFunctionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListExecutionFunctionsRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.ListExecutionFunctionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExecutionFunctionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.functions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.ListExecutionFunctionsResponse", len)?;
        if !self.functions.is_empty() {
            struct_ser.serialize_field("functions", &self.functions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExecutionFunctionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "functions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Functions,
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
                            "functions" => Ok(GeneratedField::Functions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListExecutionFunctionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.ListExecutionFunctionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExecutionFunctionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut functions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Functions => {
                            if functions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functions"));
                            }
                            functions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListExecutionFunctionsResponse {
                    functions: functions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.ListExecutionFunctionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExecutionMethodsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.ListExecutionMethodsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExecutionMethodsRequest {
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
            type Value = ListExecutionMethodsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.ListExecutionMethodsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExecutionMethodsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListExecutionMethodsRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.ListExecutionMethodsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExecutionMethodsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.methods.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.ListExecutionMethodsResponse", len)?;
        if !self.methods.is_empty() {
            struct_ser.serialize_field("methods", &self.methods)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExecutionMethodsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "methods",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Methods,
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
                            "methods" => Ok(GeneratedField::Methods),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListExecutionMethodsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.ListExecutionMethodsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExecutionMethodsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut methods__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Methods => {
                            if methods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("methods"));
                            }
                            methods__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListExecutionMethodsResponse {
                    methods: methods__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.ListExecutionMethodsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExecutionServicesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.ListExecutionServicesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExecutionServicesRequest {
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
            type Value = ListExecutionServicesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.ListExecutionServicesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExecutionServicesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListExecutionServicesRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.ListExecutionServicesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExecutionServicesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.services.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.ListExecutionServicesResponse", len)?;
        if !self.services.is_empty() {
            struct_ser.serialize_field("services", &self.services)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExecutionServicesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "services",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Services,
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
                            "services" => Ok(GeneratedField::Services),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListExecutionServicesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.ListExecutionServicesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExecutionServicesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut services__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Services => {
                            if services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("services"));
                            }
                            services__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListExecutionServicesResponse {
                    services: services__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.ListExecutionServicesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PatchTarget {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        if self.endpoint.is_some() {
            len += 1;
        }
        if self.expiration_signing_key.is_some() {
            len += 1;
        }
        if self.target_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.PatchTarget", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if let Some(v) = self.endpoint.as_ref() {
            struct_ser.serialize_field("endpoint", v)?;
        }
        if let Some(v) = self.expiration_signing_key.as_ref() {
            struct_ser.serialize_field("expirationSigningKey", v)?;
        }
        if let Some(v) = self.target_type.as_ref() {
            match v {
                patch_target::TargetType::RestWebhook(v) => {
                    struct_ser.serialize_field("restWebhook", v)?;
                }
                patch_target::TargetType::RestCall(v) => {
                    struct_ser.serialize_field("restCall", v)?;
                }
                patch_target::TargetType::RestAsync(v) => {
                    struct_ser.serialize_field("restAsync", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PatchTarget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "timeout",
            "endpoint",
            "expiration_signing_key",
            "expirationSigningKey",
            "rest_webhook",
            "restWebhook",
            "rest_call",
            "restCall",
            "rest_async",
            "restAsync",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Timeout,
            Endpoint,
            ExpirationSigningKey,
            RestWebhook,
            RestCall,
            RestAsync,
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
                            "name" => Ok(GeneratedField::Name),
                            "timeout" => Ok(GeneratedField::Timeout),
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "expirationSigningKey" | "expiration_signing_key" => Ok(GeneratedField::ExpirationSigningKey),
                            "restWebhook" | "rest_webhook" => Ok(GeneratedField::RestWebhook),
                            "restCall" | "rest_call" => Ok(GeneratedField::RestCall),
                            "restAsync" | "rest_async" => Ok(GeneratedField::RestAsync),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PatchTarget;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.PatchTarget")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PatchTarget, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut timeout__ = None;
                let mut endpoint__ = None;
                let mut expiration_signing_key__ = None;
                let mut target_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map_.next_value()?;
                        }
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = map_.next_value()?;
                        }
                        GeneratedField::ExpirationSigningKey => {
                            if expiration_signing_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationSigningKey"));
                            }
                            expiration_signing_key__ = map_.next_value()?;
                        }
                        GeneratedField::RestWebhook => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restWebhook"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(patch_target::TargetType::RestWebhook)
;
                        }
                        GeneratedField::RestCall => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restCall"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(patch_target::TargetType::RestCall)
;
                        }
                        GeneratedField::RestAsync => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restAsync"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(patch_target::TargetType::RestAsync)
;
                        }
                    }
                }
                Ok(PatchTarget {
                    name: name__,
                    timeout: timeout__,
                    endpoint: endpoint__,
                    expiration_signing_key: expiration_signing_key__,
                    target_type: target_type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.PatchTarget", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PatchTargetRequest {
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
        if self.target.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.PatchTargetRequest", len)?;
        if let Some(v) = self.instance.as_ref() {
            struct_ser.serialize_field("instance", v)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.target.as_ref() {
            struct_ser.serialize_field("target", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PatchTargetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance",
            "id",
            "target",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Instance,
            Id,
            Target,
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
                            "target" => Ok(GeneratedField::Target),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PatchTargetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.PatchTargetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PatchTargetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance__ = None;
                let mut id__ = None;
                let mut target__ = None;
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
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PatchTargetRequest {
                    instance: instance__,
                    id: id__.unwrap_or_default(),
                    target: target__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.PatchTargetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PatchTargetResponse {
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
        if self.signing_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.PatchTargetResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.signing_key.as_ref() {
            struct_ser.serialize_field("signingKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PatchTargetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "signing_key",
            "signingKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            SigningKey,
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
                            "signingKey" | "signing_key" => Ok(GeneratedField::SigningKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PatchTargetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.PatchTargetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PatchTargetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut signing_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::SigningKey => {
                            if signing_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signingKey"));
                            }
                            signing_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PatchTargetResponse {
                    details: details__,
                    signing_key: signing_key__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.PatchTargetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestExecution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.condition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.RequestExecution", len)?;
        if let Some(v) = self.condition.as_ref() {
            match v {
                request_execution::Condition::Method(v) => {
                    struct_ser.serialize_field("method", v)?;
                }
                request_execution::Condition::Service(v) => {
                    struct_ser.serialize_field("service", v)?;
                }
                request_execution::Condition::All(v) => {
                    struct_ser.serialize_field("all", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestExecution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "method",
            "service",
            "all",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Method,
            Service,
            All,
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
                            "method" => Ok(GeneratedField::Method),
                            "service" => Ok(GeneratedField::Service),
                            "all" => Ok(GeneratedField::All),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestExecution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.RequestExecution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestExecution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut condition__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Method => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(request_execution::Condition::Method);
                        }
                        GeneratedField::Service => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(request_execution::Condition::Service);
                        }
                        GeneratedField::All => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("all"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(request_execution::Condition::All);
                        }
                    }
                }
                Ok(RequestExecution {
                    condition: condition__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.RequestExecution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseExecution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.condition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.ResponseExecution", len)?;
        if let Some(v) = self.condition.as_ref() {
            match v {
                response_execution::Condition::Method(v) => {
                    struct_ser.serialize_field("method", v)?;
                }
                response_execution::Condition::Service(v) => {
                    struct_ser.serialize_field("service", v)?;
                }
                response_execution::Condition::All(v) => {
                    struct_ser.serialize_field("all", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseExecution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "method",
            "service",
            "all",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Method,
            Service,
            All,
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
                            "method" => Ok(GeneratedField::Method),
                            "service" => Ok(GeneratedField::Service),
                            "all" => Ok(GeneratedField::All),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseExecution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.ResponseExecution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseExecution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut condition__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Method => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(response_execution::Condition::Method);
                        }
                        GeneratedField::Service => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(response_execution::Condition::Service);
                        }
                        GeneratedField::All => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("all"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(response_execution::Condition::All);
                        }
                    }
                }
                Ok(ResponseExecution {
                    condition: condition__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.ResponseExecution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchExecutionsRequest {
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
        if self.query.is_some() {
            len += 1;
        }
        if self.sorting_column.is_some() {
            len += 1;
        }
        if !self.filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.SearchExecutionsRequest", len)?;
        if let Some(v) = self.instance.as_ref() {
            struct_ser.serialize_field("instance", v)?;
        }
        if let Some(v) = self.query.as_ref() {
            struct_ser.serialize_field("query", v)?;
        }
        if let Some(v) = self.sorting_column.as_ref() {
            let v = ExecutionFieldName::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchExecutionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance",
            "query",
            "sorting_column",
            "sortingColumn",
            "filters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Instance,
            Query,
            SortingColumn,
            Filters,
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
                            "query" => Ok(GeneratedField::Query),
                            "sortingColumn" | "sorting_column" => Ok(GeneratedField::SortingColumn),
                            "filters" => Ok(GeneratedField::Filters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchExecutionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.SearchExecutionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchExecutionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance__ = None;
                let mut query__ = None;
                let mut sorting_column__ = None;
                let mut filters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Instance => {
                            if instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            instance__ = map_.next_value()?;
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = map_.next_value()?;
                        }
                        GeneratedField::SortingColumn => {
                            if sorting_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortingColumn"));
                            }
                            sorting_column__ = map_.next_value::<::std::option::Option<ExecutionFieldName>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SearchExecutionsRequest {
                    instance: instance__,
                    query: query__,
                    sorting_column: sorting_column__,
                    filters: filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.SearchExecutionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchExecutionsResponse {
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
        if !self.result.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.SearchExecutionsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.result.is_empty() {
            struct_ser.serialize_field("result", &self.result)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchExecutionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchExecutionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.SearchExecutionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchExecutionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SearchExecutionsResponse {
                    details: details__,
                    result: result__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.SearchExecutionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchTargetsRequest {
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
        if self.query.is_some() {
            len += 1;
        }
        if self.sorting_column.is_some() {
            len += 1;
        }
        if !self.filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.SearchTargetsRequest", len)?;
        if let Some(v) = self.instance.as_ref() {
            struct_ser.serialize_field("instance", v)?;
        }
        if let Some(v) = self.query.as_ref() {
            struct_ser.serialize_field("query", v)?;
        }
        if let Some(v) = self.sorting_column.as_ref() {
            let v = TargetFieldName::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchTargetsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance",
            "query",
            "sorting_column",
            "sortingColumn",
            "filters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Instance,
            Query,
            SortingColumn,
            Filters,
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
                            "query" => Ok(GeneratedField::Query),
                            "sortingColumn" | "sorting_column" => Ok(GeneratedField::SortingColumn),
                            "filters" => Ok(GeneratedField::Filters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchTargetsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.SearchTargetsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchTargetsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance__ = None;
                let mut query__ = None;
                let mut sorting_column__ = None;
                let mut filters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Instance => {
                            if instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            instance__ = map_.next_value()?;
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = map_.next_value()?;
                        }
                        GeneratedField::SortingColumn => {
                            if sorting_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortingColumn"));
                            }
                            sorting_column__ = map_.next_value::<::std::option::Option<TargetFieldName>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SearchTargetsRequest {
                    instance: instance__,
                    query: query__,
                    sorting_column: sorting_column__,
                    filters: filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.SearchTargetsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchTargetsResponse {
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
        if !self.result.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.SearchTargetsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.result.is_empty() {
            struct_ser.serialize_field("result", &self.result)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchTargetsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchTargetsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.SearchTargetsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchTargetsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SearchTargetsResponse {
                    details: details__,
                    result: result__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.SearchTargetsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetExecutionRequest {
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
        if self.condition.is_some() {
            len += 1;
        }
        if self.execution.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.SetExecutionRequest", len)?;
        if let Some(v) = self.instance.as_ref() {
            struct_ser.serialize_field("instance", v)?;
        }
        if let Some(v) = self.condition.as_ref() {
            struct_ser.serialize_field("condition", v)?;
        }
        if let Some(v) = self.execution.as_ref() {
            struct_ser.serialize_field("execution", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetExecutionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance",
            "condition",
            "execution",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Instance,
            Condition,
            Execution,
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
                            "condition" => Ok(GeneratedField::Condition),
                            "execution" => Ok(GeneratedField::Execution),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetExecutionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.SetExecutionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetExecutionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance__ = None;
                let mut condition__ = None;
                let mut execution__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Instance => {
                            if instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            instance__ = map_.next_value()?;
                        }
                        GeneratedField::Condition => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            condition__ = map_.next_value()?;
                        }
                        GeneratedField::Execution => {
                            if execution__.is_some() {
                                return Err(serde::de::Error::duplicate_field("execution"));
                            }
                            execution__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetExecutionRequest {
                    instance: instance__,
                    condition: condition__,
                    execution: execution__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.SetExecutionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetExecutionResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.SetExecutionResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetExecutionResponse {
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
            type Value = SetExecutionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.SetExecutionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetExecutionResponse, V::Error>
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
                Ok(SetExecutionResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.SetExecutionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetRestAsync {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.SetRESTAsync", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetRestAsync {
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
            type Value = SetRestAsync;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.SetRESTAsync")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetRestAsync, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SetRestAsync {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.SetRESTAsync", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetRestCall {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.interrupt_on_error {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.SetRESTCall", len)?;
        if self.interrupt_on_error {
            struct_ser.serialize_field("interruptOnError", &self.interrupt_on_error)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetRestCall {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "interrupt_on_error",
            "interruptOnError",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InterruptOnError,
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
                            "interruptOnError" | "interrupt_on_error" => Ok(GeneratedField::InterruptOnError),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetRestCall;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.SetRESTCall")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetRestCall, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut interrupt_on_error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InterruptOnError => {
                            if interrupt_on_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interruptOnError"));
                            }
                            interrupt_on_error__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetRestCall {
                    interrupt_on_error: interrupt_on_error__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.SetRESTCall", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetRestWebhook {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.interrupt_on_error {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.SetRESTWebhook", len)?;
        if self.interrupt_on_error {
            struct_ser.serialize_field("interruptOnError", &self.interrupt_on_error)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetRestWebhook {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "interrupt_on_error",
            "interruptOnError",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InterruptOnError,
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
                            "interruptOnError" | "interrupt_on_error" => Ok(GeneratedField::InterruptOnError),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetRestWebhook;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.SetRESTWebhook")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetRestWebhook, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut interrupt_on_error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InterruptOnError => {
                            if interrupt_on_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interruptOnError"));
                            }
                            interrupt_on_error__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetRestWebhook {
                    interrupt_on_error: interrupt_on_error__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.SetRESTWebhook", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Target {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        if !self.endpoint.is_empty() {
            len += 1;
        }
        if self.target_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.Target", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if !self.endpoint.is_empty() {
            struct_ser.serialize_field("endpoint", &self.endpoint)?;
        }
        if let Some(v) = self.target_type.as_ref() {
            match v {
                target::TargetType::RestWebhook(v) => {
                    struct_ser.serialize_field("restWebhook", v)?;
                }
                target::TargetType::RestCall(v) => {
                    struct_ser.serialize_field("restCall", v)?;
                }
                target::TargetType::RestAsync(v) => {
                    struct_ser.serialize_field("restAsync", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Target {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "timeout",
            "endpoint",
            "rest_webhook",
            "restWebhook",
            "rest_call",
            "restCall",
            "rest_async",
            "restAsync",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Timeout,
            Endpoint,
            RestWebhook,
            RestCall,
            RestAsync,
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
                            "name" => Ok(GeneratedField::Name),
                            "timeout" => Ok(GeneratedField::Timeout),
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "restWebhook" | "rest_webhook" => Ok(GeneratedField::RestWebhook),
                            "restCall" | "rest_call" => Ok(GeneratedField::RestCall),
                            "restAsync" | "rest_async" => Ok(GeneratedField::RestAsync),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Target;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.Target")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Target, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut timeout__ = None;
                let mut endpoint__ = None;
                let mut target_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map_.next_value()?;
                        }
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RestWebhook => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restWebhook"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(target::TargetType::RestWebhook)
;
                        }
                        GeneratedField::RestCall => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restCall"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(target::TargetType::RestCall)
;
                        }
                        GeneratedField::RestAsync => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restAsync"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(target::TargetType::RestAsync)
;
                        }
                    }
                }
                Ok(Target {
                    name: name__.unwrap_or_default(),
                    timeout: timeout__,
                    endpoint: endpoint__.unwrap_or_default(),
                    target_type: target_type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.Target", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TargetFieldName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TARGET_FIELD_NAME_UNSPECIFIED",
            Self::Id => "TARGET_FIELD_NAME_ID",
            Self::CreatedDate => "TARGET_FIELD_NAME_CREATED_DATE",
            Self::ChangedDate => "TARGET_FIELD_NAME_CHANGED_DATE",
            Self::Name => "TARGET_FIELD_NAME_NAME",
            Self::TargetType => "TARGET_FIELD_NAME_TARGET_TYPE",
            Self::Url => "TARGET_FIELD_NAME_URL",
            Self::Timeout => "TARGET_FIELD_NAME_TIMEOUT",
            Self::InterruptOnError => "TARGET_FIELD_NAME_INTERRUPT_ON_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TargetFieldName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TARGET_FIELD_NAME_UNSPECIFIED",
            "TARGET_FIELD_NAME_ID",
            "TARGET_FIELD_NAME_CREATED_DATE",
            "TARGET_FIELD_NAME_CHANGED_DATE",
            "TARGET_FIELD_NAME_NAME",
            "TARGET_FIELD_NAME_TARGET_TYPE",
            "TARGET_FIELD_NAME_URL",
            "TARGET_FIELD_NAME_TIMEOUT",
            "TARGET_FIELD_NAME_INTERRUPT_ON_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TargetFieldName;

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
                    "TARGET_FIELD_NAME_UNSPECIFIED" => Ok(TargetFieldName::Unspecified),
                    "TARGET_FIELD_NAME_ID" => Ok(TargetFieldName::Id),
                    "TARGET_FIELD_NAME_CREATED_DATE" => Ok(TargetFieldName::CreatedDate),
                    "TARGET_FIELD_NAME_CHANGED_DATE" => Ok(TargetFieldName::ChangedDate),
                    "TARGET_FIELD_NAME_NAME" => Ok(TargetFieldName::Name),
                    "TARGET_FIELD_NAME_TARGET_TYPE" => Ok(TargetFieldName::TargetType),
                    "TARGET_FIELD_NAME_URL" => Ok(TargetFieldName::Url),
                    "TARGET_FIELD_NAME_TIMEOUT" => Ok(TargetFieldName::Timeout),
                    "TARGET_FIELD_NAME_INTERRUPT_ON_ERROR" => Ok(TargetFieldName::InterruptOnError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TargetFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.target_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.TargetFilter", len)?;
        if !self.target_id.is_empty() {
            struct_ser.serialize_field("targetId", &self.target_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TargetFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target_id",
            "targetId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TargetId,
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
                            "targetId" | "target_id" => Ok(GeneratedField::TargetId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TargetFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.TargetFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TargetFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TargetId => {
                            if target_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetId"));
                            }
                            target_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TargetFilter {
                    target_id: target_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.TargetFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TargetNameFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.target_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.TargetNameFilter", len)?;
        if !self.target_name.is_empty() {
            struct_ser.serialize_field("targetName", &self.target_name)?;
        }
        if self.method != 0 {
            let v = super::super::object::v3alpha::TextFilterMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TargetNameFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target_name",
            "targetName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TargetName,
            Method,
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
                            "targetName" | "target_name" => Ok(GeneratedField::TargetName),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TargetNameFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.TargetNameFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TargetNameFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TargetName => {
                            if target_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetName"));
                            }
                            target_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::object::v3alpha::TextFilterMethod>()? as i32);
                        }
                    }
                }
                Ok(TargetNameFilter {
                    target_name: target_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.TargetNameFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TargetSearchFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.action.v3alpha.TargetSearchFilter", len)?;
        if let Some(v) = self.filter.as_ref() {
            match v {
                target_search_filter::Filter::TargetNameFilter(v) => {
                    struct_ser.serialize_field("targetNameFilter", v)?;
                }
                target_search_filter::Filter::InTargetIdsFilter(v) => {
                    struct_ser.serialize_field("inTargetIdsFilter", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TargetSearchFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target_name_filter",
            "targetNameFilter",
            "in_target_ids_filter",
            "inTargetIdsFilter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TargetNameFilter,
            InTargetIdsFilter,
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
                            "targetNameFilter" | "target_name_filter" => Ok(GeneratedField::TargetNameFilter),
                            "inTargetIdsFilter" | "in_target_ids_filter" => Ok(GeneratedField::InTargetIdsFilter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TargetSearchFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.action.v3alpha.TargetSearchFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TargetSearchFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TargetNameFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetNameFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(target_search_filter::Filter::TargetNameFilter)
;
                        }
                        GeneratedField::InTargetIdsFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inTargetIdsFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(target_search_filter::Filter::InTargetIdsFilter)
;
                        }
                    }
                }
                Ok(TargetSearchFilter {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.action.v3alpha.TargetSearchFilter", FIELDS, GeneratedVisitor)
    }
}
