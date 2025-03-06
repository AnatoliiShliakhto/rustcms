BEGIN TRANSACTION;

LET $su_login = 'root';
LET $su_password = 'root';

DEFINE TABLE core_accounts SCHEMAFULL;
DEFINE FIELD login ON TABLE core_accounts TYPE string;
DEFINE FIELD password ON TABLE core_accounts TYPE option<string>;
DEFINE FIELD blocked ON TABLE core_accounts TYPE bool DEFAULT false;
DEFINE FIELD metadata ON TABLE core_accounts TYPE {
    created_at: int,
    updated_at: int,
    created_by: string,
    updated_by: string,
} DEFAULT {
    created_at: time::unix(),
    updated_at: time::unix(),
    created_by: 'system',
    updated_by: 'system',
};
DEFINE FIELD metadata.updated_at ON TABLE core_accounts TYPE int VALUE time::unix();
DEFINE INDEX idx_core_accounts_login ON TABLE core_accounts COLUMNS login UNIQUE;
LET $su_rec = CREATE ONLY core_accounts CONTENT {
    login: $su_login,
    password: crypto::argon2::generate($su_password)
} RETURN id;

DEFINE TABLE core_refresh_tokens SCHEMAFULL;
DEFINE FIELD refresh_token ON TABLE core_refresh_tokens TYPE uuid VALUE rand::uuid::v4();
DEFINE FIELD issued_at ON TABLE core_refresh_tokens TYPE int VALUE time::unix();
DEFINE FIELD expiration_at ON TABLE core_refresh_tokens TYPE int;
DEFINE FIELD device ON TABLE core_refresh_tokens TYPE option<string>;
DEFINE INDEX idx_core_refresh_tokens_token ON TABLE core_refresh_tokens COLUMNS refresh_token;

DEFINE TABLE rel_account_refresh_tokens SCHEMAFULL TYPE RELATION IN core_accounts OUT core_refresh_tokens;
DEFINE INDEX idx_rel_account_refresh_tokens ON TABLE rel_account_refresh_tokens COLUMNS in, out UNIQUE;

DEFINE TABLE core_roles SCHEMAFULL;
DEFINE FIELD name ON TABLE core_roles TYPE string;
DEFINE FIELD custom ON TABLE core_roles TYPE bool DEFAULT true;
DEFINE FIELD metadata ON TABLE core_roles TYPE {
    created_at: int,
    updated_at: int,
    created_by: string,
    updated_by: string,
} DEFAULT {
    created_at: time::unix(),
    updated_at: time::unix(),
    created_by: 'system',
    updated_by: 'system',
};
DEFINE FIELD metadata.updated_at ON TABLE core_roles TYPE int VALUE time::unix();

LET $role_admin_rec = CREATE ONLY core_roles CONTENT {
    name: "Administrator",
    custom: false
} RETURN id;

DEFINE TABLE rel_account_roles SCHEMAFULL TYPE RELATION IN core_accounts OUT core_roles;
DEFINE FIELD metadata ON TABLE rel_account_roles TYPE {
    created_at: int,
    updated_at: int,
    created_by: string,
    updated_by: string,
} DEFAULT {
    created_at: time::unix(),
    updated_at: time::unix(),
    created_by: 'system',
    updated_by: 'system',
};
DEFINE FIELD metadata.updated_at ON TABLE rel_account_roles TYPE int VALUE time::unix();
DEFINE INDEX idx_rel_account_roles ON TABLE rel_account_roles COLUMNS in, out UNIQUE;

RELATE $su_rec->rel_account_roles->$role_admin_rec;

DEFINE TABLE core_permissions SCHEMAFULL;
DEFINE FIELD name ON TABLE core_permissions TYPE string;
DEFINE FIELD custom ON TABLE core_permissions TYPE bool DEFAULT true;
DEFINE FIELD metadata ON TABLE core_permissions TYPE {
    created_at: int,
    updated_at: int,
    created_by: string,
    updated_by: string,
} DEFAULT {
    created_at: time::unix(),
    updated_at: time::unix(),
    created_by: 'system',
    updated_by: 'system',
};
DEFINE FIELD metadata.updated_at ON TABLE core_permissions TYPE int VALUE time::unix();

LET $permission_system_rec = CREATE ONLY core_permissions CONTENT {
    name: "System",
    custom: false
} RETURN id;
LET $permission_account_rec = CREATE ONLY core_permissions CONTENT {
    name: "Accounts",
    custom: false
} RETURN id;
LET $permission_role_rec = CREATE ONLY core_permissions CONTENT {
    name: "Roles",
    custom: false
} RETURN id;
LET $permission_permission_rec = CREATE ONLY core_permissions CONTENT {
    name: "Permissions",
    custom: false
} RETURN id;

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
    created_by: string,
    updated_by: string,
} DEFAULT {
    created_at: time::unix(),
    updated_at: time::unix(),
    created_by: 'system',
    updated_by: 'system',
};
DEFINE FIELD metadata.updated_at ON TABLE rel_role_permissions TYPE int VALUE time::unix();
DEFINE INDEX idx_rel_role_permissions ON TABLE rel_role_permissions COLUMNS in, out UNIQUE;

RELATE $role_admin_rec->rel_role_permissions->$permission_system_rec CONTENT {
    capabilities: {
        create: true,
        view: true,
        edit: true,
        delete: true
    }
};
RELATE $role_admin_rec->rel_role_permissions->$permission_account_rec CONTENT {
    capabilities: {
        create: true,
        view: true,
        edit: true,
        delete: true
    }
};
RELATE $role_admin_rec->rel_role_permissions->$permission_role_rec CONTENT {
    capabilities: {
        create: true,
        view: true,
        edit: true,
        delete: true
    }
};
RELATE $role_admin_rec->rel_role_permissions->$permission_permission_rec CONTENT {
    capabilities: {
        create: true,
        view: true,
        edit: true,
        delete: true
    }
};

RETURN 1;

COMMIT TRANSACTION;