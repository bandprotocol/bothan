// @generated
impl serde::Serialize for GetInfoRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("bothan.v1.GetInfoRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetInfoRequest {
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
            type Value = GetInfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bothan.v1.GetInfoRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetInfoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetInfoRequest {
                })
            }
        }
        deserializer.deserialize_struct("bothan.v1.GetInfoRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetInfoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bothan_version.is_empty() {
            len += 1;
        }
        if !self.registry_ipfs_hash.is_empty() {
            len += 1;
        }
        if !self.registry_version_requirement.is_empty() {
            len += 1;
        }
        if !self.active_sources.is_empty() {
            len += 1;
        }
        if self.monitoring_enabled {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bothan.v1.GetInfoResponse", len)?;
        if !self.bothan_version.is_empty() {
            struct_ser.serialize_field("bothanVersion", &self.bothan_version)?;
        }
        if !self.registry_ipfs_hash.is_empty() {
            struct_ser.serialize_field("registryIpfsHash", &self.registry_ipfs_hash)?;
        }
        if !self.registry_version_requirement.is_empty() {
            struct_ser.serialize_field("registryVersionRequirement", &self.registry_version_requirement)?;
        }
        if !self.active_sources.is_empty() {
            struct_ser.serialize_field("activeSources", &self.active_sources)?;
        }
        if self.monitoring_enabled {
            struct_ser.serialize_field("monitoringEnabled", &self.monitoring_enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetInfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bothan_version",
            "bothanVersion",
            "registry_ipfs_hash",
            "registryIpfsHash",
            "registry_version_requirement",
            "registryVersionRequirement",
            "active_sources",
            "activeSources",
            "monitoring_enabled",
            "monitoringEnabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BothanVersion,
            RegistryIpfsHash,
            RegistryVersionRequirement,
            ActiveSources,
            MonitoringEnabled,
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
                            "bothanVersion" | "bothan_version" => Ok(GeneratedField::BothanVersion),
                            "registryIpfsHash" | "registry_ipfs_hash" => Ok(GeneratedField::RegistryIpfsHash),
                            "registryVersionRequirement" | "registry_version_requirement" => Ok(GeneratedField::RegistryVersionRequirement),
                            "activeSources" | "active_sources" => Ok(GeneratedField::ActiveSources),
                            "monitoringEnabled" | "monitoring_enabled" => Ok(GeneratedField::MonitoringEnabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetInfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bothan.v1.GetInfoResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetInfoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bothan_version__ = None;
                let mut registry_ipfs_hash__ = None;
                let mut registry_version_requirement__ = None;
                let mut active_sources__ = None;
                let mut monitoring_enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BothanVersion => {
                            if bothan_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bothanVersion"));
                            }
                            bothan_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RegistryIpfsHash => {
                            if registry_ipfs_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registryIpfsHash"));
                            }
                            registry_ipfs_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RegistryVersionRequirement => {
                            if registry_version_requirement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registryVersionRequirement"));
                            }
                            registry_version_requirement__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ActiveSources => {
                            if active_sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeSources"));
                            }
                            active_sources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MonitoringEnabled => {
                            if monitoring_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("monitoringEnabled"));
                            }
                            monitoring_enabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetInfoResponse {
                    bothan_version: bothan_version__.unwrap_or_default(),
                    registry_ipfs_hash: registry_ipfs_hash__.unwrap_or_default(),
                    registry_version_requirement: registry_version_requirement__.unwrap_or_default(),
                    active_sources: active_sources__.unwrap_or_default(),
                    monitoring_enabled: monitoring_enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bothan.v1.GetInfoResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPricesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signal_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bothan.v1.GetPricesRequest", len)?;
        if !self.signal_ids.is_empty() {
            struct_ser.serialize_field("signalIds", &self.signal_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPricesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signal_ids",
            "signalIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SignalIds,
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
                            "signalIds" | "signal_ids" => Ok(GeneratedField::SignalIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPricesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bothan.v1.GetPricesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPricesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signal_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SignalIds => {
                            if signal_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signalIds"));
                            }
                            signal_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetPricesRequest {
                    signal_ids: signal_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bothan.v1.GetPricesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPricesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uuid.is_empty() {
            len += 1;
        }
        if !self.prices.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bothan.v1.GetPricesResponse", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.prices.is_empty() {
            struct_ser.serialize_field("prices", &self.prices)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPricesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "prices",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            Prices,
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
                            "uuid" => Ok(GeneratedField::Uuid),
                            "prices" => Ok(GeneratedField::Prices),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPricesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bothan.v1.GetPricesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPricesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut prices__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Prices => {
                            if prices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prices"));
                            }
                            prices__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetPricesResponse {
                    uuid: uuid__.unwrap_or_default(),
                    prices: prices__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bothan.v1.GetPricesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Price {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signal_id.is_empty() {
            len += 1;
        }
        if self.price != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bothan.v1.Price", len)?;
        if !self.signal_id.is_empty() {
            struct_ser.serialize_field("signalId", &self.signal_id)?;
        }
        if self.price != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("price", ToString::to_string(&self.price).as_str())?;
        }
        if self.status != 0 {
            let v = Status::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Price {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signal_id",
            "signalId",
            "price",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SignalId,
            Price,
            Status,
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
                            "signalId" | "signal_id" => Ok(GeneratedField::SignalId),
                            "price" => Ok(GeneratedField::Price),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Price;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bothan.v1.Price")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Price, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signal_id__ = None;
                let mut price__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SignalId => {
                            if signal_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signalId"));
                            }
                            signal_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<Status>()? as i32);
                        }
                    }
                }
                Ok(Price {
                    signal_id: signal_id__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bothan.v1.Price", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PushMonitoringRecordsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uuid.is_empty() {
            len += 1;
        }
        if !self.tx_hash.is_empty() {
            len += 1;
        }
        if !self.signal_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bothan.v1.PushMonitoringRecordsRequest", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.tx_hash.is_empty() {
            struct_ser.serialize_field("txHash", &self.tx_hash)?;
        }
        if !self.signal_ids.is_empty() {
            struct_ser.serialize_field("signalIds", &self.signal_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PushMonitoringRecordsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "tx_hash",
            "txHash",
            "signal_ids",
            "signalIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            TxHash,
            SignalIds,
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
                            "uuid" => Ok(GeneratedField::Uuid),
                            "txHash" | "tx_hash" => Ok(GeneratedField::TxHash),
                            "signalIds" | "signal_ids" => Ok(GeneratedField::SignalIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PushMonitoringRecordsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bothan.v1.PushMonitoringRecordsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PushMonitoringRecordsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut tx_hash__ = None;
                let mut signal_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TxHash => {
                            if tx_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txHash"));
                            }
                            tx_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SignalIds => {
                            if signal_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signalIds"));
                            }
                            signal_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PushMonitoringRecordsRequest {
                    uuid: uuid__.unwrap_or_default(),
                    tx_hash: tx_hash__.unwrap_or_default(),
                    signal_ids: signal_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bothan.v1.PushMonitoringRecordsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PushMonitoringRecordsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("bothan.v1.PushMonitoringRecordsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PushMonitoringRecordsResponse {
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
            type Value = PushMonitoringRecordsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bothan.v1.PushMonitoringRecordsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PushMonitoringRecordsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PushMonitoringRecordsResponse {
                })
            }
        }
        deserializer.deserialize_struct("bothan.v1.PushMonitoringRecordsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Status {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STATUS_UNSPECIFIED",
            Self::Unsupported => "STATUS_UNSUPPORTED",
            Self::Unavailable => "STATUS_UNAVAILABLE",
            Self::Available => "STATUS_AVAILABLE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Status {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATUS_UNSPECIFIED",
            "STATUS_UNSUPPORTED",
            "STATUS_UNAVAILABLE",
            "STATUS_AVAILABLE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Status;

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
                    "STATUS_UNSPECIFIED" => Ok(Status::Unspecified),
                    "STATUS_UNSUPPORTED" => Ok(Status::Unsupported),
                    "STATUS_UNAVAILABLE" => Ok(Status::Unavailable),
                    "STATUS_AVAILABLE" => Ok(Status::Available),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRegistryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ipfs_hash.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bothan.v1.UpdateRegistryRequest", len)?;
        if !self.ipfs_hash.is_empty() {
            struct_ser.serialize_field("ipfsHash", &self.ipfs_hash)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRegistryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ipfs_hash",
            "ipfsHash",
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IpfsHash,
            Version,
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
                            "ipfsHash" | "ipfs_hash" => Ok(GeneratedField::IpfsHash),
                            "version" => Ok(GeneratedField::Version),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRegistryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bothan.v1.UpdateRegistryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRegistryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ipfs_hash__ = None;
                let mut version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IpfsHash => {
                            if ipfs_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipfsHash"));
                            }
                            ipfs_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateRegistryRequest {
                    ipfs_hash: ipfs_hash__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bothan.v1.UpdateRegistryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRegistryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("bothan.v1.UpdateRegistryResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRegistryResponse {
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
            type Value = UpdateRegistryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bothan.v1.UpdateRegistryResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRegistryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UpdateRegistryResponse {
                })
            }
        }
        deserializer.deserialize_struct("bothan.v1.UpdateRegistryResponse", FIELDS, GeneratedVisitor)
    }
}
