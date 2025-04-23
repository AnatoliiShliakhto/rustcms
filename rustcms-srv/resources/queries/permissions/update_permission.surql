BEGIN TRANSACTION;

LET $update = UPDATE type::thing('core_permissions', $permission_id) MERGE {
    name: $permission_name,
    metadata: {
        updated_by: type::thing('core_accounts', $by_id),
        updated_at: time::unix()
    }
} WHERE custom = true;

RETURN IF $update.len() == 0 THEN { none } ELSE {
    {
        id: $update[0].id.id(),
        name: $update[0].name,
        custom: $update[0].custom,
        metadata: fn::metadata($update[0].metadata)
    };
} END;

COMMIT TRANSACTION;