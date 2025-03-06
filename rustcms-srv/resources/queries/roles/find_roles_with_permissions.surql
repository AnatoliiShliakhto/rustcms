SELECT
    id.id() as id,
    name,
    (->rel_role_permissions.{out, capabilities})
        .map(|$item|
            {
                "id": $item.out.id(),
                "name": $item.out.name,
                "create": $item.capabilities.create,
                "view": $item.capabilities.view,
                "edit": $item.capabilities.edit,
                "delete": $item.capabilities.delete
            }
        ) as permissions
FROM core_roles;