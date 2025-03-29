pub mod shared {
    use crate::js::types::candid::JsRawPrincipal;
    use junobuild_collections::types::core::CollectionKey;
    use junobuild_shared::types::state::{Timestamp, Version};

    pub type JsCollectionKey = CollectionKey;

    pub type JsTimestamp = Timestamp;
    pub type JsVersion = Version;

    pub type JsUserId<'js> = JsRawPrincipal<'js>;
}

pub mod hooks {
    use crate::hooks::js::types::db::JsDoc;
    use crate::hooks::js::types::interface::{JsDelDoc, JsSetDoc};
    use crate::hooks::js::types::shared::{JsCollectionKey, JsUserId};
    use crate::js::types::candid::JsUint8Array;
    use junobuild_shared::types::core::Key;

    pub type JsKey = Key;

    pub type JsRawData<'js> = JsUint8Array<'js>;

    #[derive(Clone)]
    pub struct JsHookContext<'js, T> {
        pub caller: JsUserId<'js>,
        pub data: T,
    }

    #[derive(Clone)]
    pub struct JsDocContext<T> {
        pub collection: JsCollectionKey,
        pub key: JsKey,
        pub data: T,
    }

    #[derive(Clone)]
    pub struct JsDocUpsert<'js> {
        pub before: Option<JsDoc<'js>>,
        pub after: JsDoc<'js>,
    }

    #[derive(Clone)]
    pub struct JsDocAssertSet<'js> {
        pub current: Option<JsDoc<'js>>,
        pub proposed: JsSetDoc<'js>,
    }

    #[derive(Clone)]
    pub struct JsDocAssertDelete<'js> {
        pub current: Option<JsDoc<'js>>,
        pub proposed: JsDelDoc,
    }
}

pub mod db {
    use crate::hooks::js::types::hooks::JsRawData;
    use crate::hooks::js::types::shared::{JsTimestamp, JsUserId, JsVersion};

    #[derive(Clone)]
    pub struct JsDoc<'js> {
        pub owner: JsUserId<'js>,
        pub data: JsRawData<'js>,
        pub description: Option<String>,
        pub created_at: JsTimestamp,
        pub updated_at: JsTimestamp,
        pub version: Option<JsVersion>,
    }
}

pub mod interface {
    use crate::hooks::js::types::hooks::JsRawData;
    use crate::hooks::js::types::shared::JsVersion;
    use junobuild_shared::types::state::Version;

    #[derive(Clone)]
    pub struct JsSetDoc<'js> {
        pub data: JsRawData<'js>,
        pub description: Option<String>,
        pub version: Option<JsVersion>,
    }

    #[derive(Clone)]
    pub struct JsDelDoc {
        pub version: Option<Version>,
    }
}
