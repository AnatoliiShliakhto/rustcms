SELECT
    id.id() as id,
    name,
    (->rel_role_permissions.{out, capabilities})
        .map(|$item| {
            {
                id: $item.out.id.id(),
                name: $item.out.name,
                create: $item.capabilities.create,
                view: $item.capabilities.view,
                edit: $item.capabilities.edit,
                delete: $item.capabilities.delete
            }
        }) as permissions,
    fn::metadata(metadata) as metadata
FROM core_roles;