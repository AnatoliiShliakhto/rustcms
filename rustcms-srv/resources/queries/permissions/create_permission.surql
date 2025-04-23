BEGIN TRANSACTION;

LET $metadata = fn::metadata::create($by_id);

LET $permission = CREATE ONLY core_permissions CONTENT {
    name: $permission_name,
    metadata: $metadata
};

LET $permission_rec = $permission.id;

RELATE $role_administrator_rec->rel_role_permissions->$permission_rec CONTENT {
    capabilities: {
        create: true,
        view: true,
        edit: true,
        delete: true
    },
    metadata: $metadata
};

RETURN {
    id: $permission.id.id(),
    name: $permission.name,
    custom: $permission.custom,
    metadata: fn::metadata($permission.metadata)
};

COMMIT TRANSACTION;