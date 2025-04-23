BEGIN TRANSACTION;

LET $su_login = 'root';
LET $su_password = 'root';

DEFINE PARAM $db_migrations VALUE ['0000_init.up.surql'];

DEFINE FUNCTION OVERWRITE fn::metadata($metadata: object) {
    RETURN {
        created_by: $metadata.created_by.login,
        created_at: time::from::unix($metadata.created_at),
        updated_by: $metadata.updated_by.login,
        updated_at: time::from::unix($metadata.updated_at)
    };
};

DEFINE FUNCTION OVERWRITE fn::metadata::get($table: string, $rec_id: string) {
    LET $metadata_rec = type::thing($table, $rec_id);
    LET $metadata = IF $metadata_rec.exists() THEN { $metadata_rec } ELSE { fn::metadata::create() } END;

    RETURN {
        created_by: $metadata_rec.metadata.created_by.login,
        created_at: time::from::unix($metadata_rec.metadata.created_at),
        updated_by: $metadata_rec.metadata.updated_by.login,
        updated_at: time::from::unix($metadata_rec.metadata.updated_at)
    };
};

DEFINE FUNCTION OVERWRITE fn::metadata::rec($metadata_rec: record) {
    LET $metadata = IF $metadata_rec.exists() THEN { $metadata_rec } ELSE { fn::metadata::create() } END;

    RETURN {
        created_by: $metadata_rec.metadata.created_by.login,
        created_at: time::from::unix($metadata_rec.metadata.created_at),
        updated_by: $metadata_rec.metadata.updated_by.login,
        updated_at: time::from::unix($metadata_rec.metadata.updated_at)
    };
};

DEFINE FUNCTION OVERWRITE fn::metadata::create($by_id: option<string>) {
    LET $time_now = time::unix();
    LET $by_rec = type::thing('core_accounts', $by_id);

    RETURN {
        created_by: $by_rec,
        created_at: $time_now,
        updated_by: $by_rec,
        updated_at: $time_now
    };
};

DEFINE FUNCTION OVERWRITE fn::metadata::update($rec: record, $by_id: option<string>) {
    UPDATE $rec MERGE {
        metadata: {
            updated_by: $by_rec,
            updated_at: time::unix()
        }
    };
};

DEFINE TABLE core_accounts SCHEMAFULL;
DEFINE FIELD login ON TABLE core_accounts TYPE string;
DEFINE FIELD password ON TABLE core_accounts TYPE option<string>;
DEFINE FIELD blocked ON TABLE core_accounts TYPE bool DEFAULT false;
DEFINE FIELD metadata ON TABLE core_accounts TYPE {
    created_at: int,
    updated_at: int,
    created_by: option<record>,
    updated_by: option<record>,
} DEFAULT fn::metadata::create();
DEFINE INDEX idx_core_accounts_login ON TABLE core_accounts COLUMNS login UNIQUE;

LET $su_rec = CREATE ONLY core_accounts CONTENT {
    login: $su_login,
    password: crypto::argon2::generate($su_password)
} RETURN VALUE id;

DEFINE TABLE core_refresh_tokens SCHEMAFULL;
DEFINE FIELD refresh_token ON TABLE core_refresh_tokens TYPE uuid VALUE rand::uuid::v4();
DEFINE FIELD issued_at ON TABLE core_refresh_tokens TYPE int VALUE time::unix();
DEFINE FIELD expiration_at ON TABLE core_refresh_tokens TYPE int;
DEFINE FIELD device ON TABLE core_refresh_tokens TYPE string DEFAULT 'web';
DEFINE INDEX idx_core_refresh_tokens_token ON TABLE core_refresh_tokens COLUMNS refresh_token;

DEFINE TABLE rel_account_refresh_tokens SCHEMAFULL TYPE RELATION IN core_accounts OUT core_refresh_tokens;
DEFINE INDEX idx_rel_account_refresh_tokens ON TABLE rel_account_refresh_tokens COLUMNS in, out UNIQUE;

DEFINE TABLE core_roles SCHEMAFULL;
DEFINE FIELD name ON TABLE core_roles TYPE string;
DEFINE FIELD custom ON TABLE core_roles TYPE bool DEFAULT true;
DEFINE FIELD metadata ON TABLE core_roles TYPE {
    created_at: int,
    updated_at: int,
    created_by: option<record>,
    updated_by: option<record>,
} DEFAULT fn::metadata::create();


LET $role_administrator_rec = CREATE ONLY core_roles CONTENT {
    name: 'Administrator',
    custom: false
} RETURN VALUE id;
DEFINE PARAM $role_administrator_rec VALUE $role_administrator_rec;

LET $role_registered_rec = CREATE ONLY core_roles CONTENT {
    name: 'Registered',
    custom: false
} RETURN VALUE id;
DEFINE PARAM $role_registered_rec VALUE $role_registered_rec;

DEFINE TABLE rel_account_roles SCHEMAFULL TYPE RELATION IN core_accounts OUT core_roles;
DEFINE FIELD metadata ON TABLE rel_account_roles TYPE {
    created_at: int,
    updated_at: int,
    created_by: option<record>,
    updated_by: option<record>,
} DEFAULT fn::metadata::create();
DEFINE INDEX idx_rel_account_roles ON TABLE rel_account_roles COLUMNS in, out UNIQUE;

RELATE $su_rec->rel_account_roles->$role_administrator_rec;

DEFINE TABLE core_permissions SCHEMAFULL;
DEFINE FIELD name ON TABLE core_permissions TYPE string;
DEFINE FIELD custom ON TABLE core_permissions TYPE bool DEFAULT true;
DEFINE FIELD metadata ON TABLE core_permissions TYPE {
    created_at: int,
    updated_at: int,
    created_by: option<record>,
    updated_by: option<record>,
} DEFAULT fn::metadata::create();

LET $permission_system_rec = CREATE ONLY core_permissions CONTENT {
    name: 'System',
    custom: false
} RETURN VALUE id;
LET $permission_accounts_rec = CREATE ONLY core_permissions CONTENT {
    name: 'Accounts',
    custom: false
} RETURN VALUE id;
LET $permission_public_rec = CREATE ONLY core_permissions CONTENT {
    name: 'Public',
    custom: false
} RETURN VALUE id;
LET $permission_storage_rec = CREATE ONLY core_permissions CONTENT {
    name: 'Storage',
    custom: false
} RETURN VALUE id;

DEFINE TABLE rel_role_permissions SCHEMAFULL TYPE RELATION IN core_roles OUT core_permissions;
DEFINE FIELD capabilities ON TABLE rel_role_permissions TYPE {
    create: bool,
    view: bool,
    edit: bool,
    delete: bool
} DEFAULT {
    create: false,
    view: false,
    edit: false,
    delete: false
};
DEFINE FIELD metadata ON TABLE rel_role_permissions TYPE {
    created_at: int,
    updated_at: int,
    created_by: option<record>,
    updated_by: option<record>,
} DEFAULT fn::metadata::create();
DEFINE INDEX idx_rel_role_permissions ON TABLE rel_role_permissions COLUMNS in, out UNIQUE;

RELATE $role_administrator_rec->rel_role_permissions->$permission_system_rec CONTENT {
    capabilities: {
        create: true,
        view: true,
        edit: true,
        delete: true
    }
};
RELATE $role_administrator_rec->rel_role_permissions->$permission_accounts_rec CONTENT {
    capabilities: {
        create: true,
        view: true,
        edit: true,
        delete: true
    }
};
RELATE $role_administrator_rec->rel_role_permissions->$permission_public_rec CONTENT {
    capabilities: {
        create: true,
        view: true,
        edit: true,
        delete: true
    }
};
RELATE $role_administrator_rec->rel_role_permissions->$permission_storage_rec CONTENT {
    capabilities: {
        create: true,
        view: true,
        edit: true,
        delete: true
    }
};
RELATE $role_registered_rec->rel_role_permissions->$permission_public_rec CONTENT {
    capabilities: {
        create: false,
        view: true,
        edit: false,
        delete: false
    }
};

RETURN true;

COMMIT TRANSACTION;