// @generated
impl serde::Serialize for AddOrganizationRequest {
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
        if !self.admins.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.org.v2beta.AddOrganizationRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.admins.is_empty() {
            struct_ser.serialize_field("admins", &self.admins)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddOrganizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "admins",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Admins,
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
                            "admins" => Ok(GeneratedField::Admins),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddOrganizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.org.v2beta.AddOrganizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddOrganizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut admins__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Admins => {
                            if admins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admins"));
                            }
                            admins__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddOrganizationRequest {
                    name: name__.unwrap_or_default(),
                    admins: admins__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.org.v2beta.AddOrganizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for add_organization_request::Admin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.roles.is_empty() {
            len += 1;
        }
        if self.user_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.org.v2beta.AddOrganizationRequest.Admin", len)?;
        if !self.roles.is_empty() {
            struct_ser.serialize_field("roles", &self.roles)?;
        }
        if let Some(v) = self.user_type.as_ref() {
            match v {
                add_organization_request::admin::UserType::UserId(v) => {
                    struct_ser.serialize_field("userId", v)?;
                }
                add_organization_request::admin::UserType::Human(v) => {
                    struct_ser.serialize_field("human", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for add_organization_request::Admin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "roles",
            "user_id",
            "userId",
            "human",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Roles,
            UserId,
            Human,
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
                            "roles" => Ok(GeneratedField::Roles),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "human" => Ok(GeneratedField::Human),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = add_organization_request::Admin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.org.v2beta.AddOrganizationRequest.Admin")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<add_organization_request::Admin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut roles__ = None;
                let mut user_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Roles => {
                            if roles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            roles__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_type__ = map_.next_value::<::std::option::Option<_>>()?.map(add_organization_request::admin::UserType::UserId);
                        }
                        GeneratedField::Human => {
                            if user_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("human"));
                            }
                            user_type__ = map_.next_value::<::std::option::Option<_>>()?.map(add_organization_request::admin::UserType::Human)
;
                        }
                    }
                }
                Ok(add_organization_request::Admin {
                    roles: roles__.unwrap_or_default(),
                    user_type: user_type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.org.v2beta.AddOrganizationRequest.Admin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddOrganizationResponse {
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
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.created_admins.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.org.v2beta.AddOrganizationResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.created_admins.is_empty() {
            struct_ser.serialize_field("createdAdmins", &self.created_admins)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddOrganizationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "organization_id",
            "organizationId",
            "created_admins",
            "createdAdmins",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            OrganizationId,
            CreatedAdmins,
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
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "createdAdmins" | "created_admins" => Ok(GeneratedField::CreatedAdmins),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddOrganizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.org.v2beta.AddOrganizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddOrganizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut organization_id__ = None;
                let mut created_admins__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAdmins => {
                            if created_admins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAdmins"));
                            }
                            created_admins__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddOrganizationResponse {
                    details: details__,
                    organization_id: organization_id__.unwrap_or_default(),
                    created_admins: created_admins__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.org.v2beta.AddOrganizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for add_organization_response::CreatedAdmin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.email_code.is_some() {
            len += 1;
        }
        if self.phone_code.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.org.v2beta.AddOrganizationResponse.CreatedAdmin", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.email_code.as_ref() {
            struct_ser.serialize_field("emailCode", v)?;
        }
        if let Some(v) = self.phone_code.as_ref() {
            struct_ser.serialize_field("phoneCode", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for add_organization_response::CreatedAdmin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "email_code",
            "emailCode",
            "phone_code",
            "phoneCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            EmailCode,
            PhoneCode,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "emailCode" | "email_code" => Ok(GeneratedField::EmailCode),
                            "phoneCode" | "phone_code" => Ok(GeneratedField::PhoneCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = add_organization_response::CreatedAdmin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.org.v2beta.AddOrganizationResponse.CreatedAdmin")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<add_organization_response::CreatedAdmin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut email_code__ = None;
                let mut phone_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EmailCode => {
                            if email_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailCode"));
                            }
                            email_code__ = map_.next_value()?;
                        }
                        GeneratedField::PhoneCode => {
                            if phone_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phoneCode"));
                            }
                            phone_code__ = map_.next_value()?;
                        }
                    }
                }
                Ok(add_organization_response::CreatedAdmin {
                    user_id: user_id__.unwrap_or_default(),
                    email_code: email_code__,
                    phone_code: phone_code__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.org.v2beta.AddOrganizationResponse.CreatedAdmin", FIELDS, GeneratedVisitor)
    }
}
