SELECT
    id.id() as id,
    name.lowercase(),
    (->rel_role_permissions.{out, capabilities})
        .map(|$item| {
            LET $create = IF $item.capabilities.create THEN RETURN $item.out.name.lowercase() + ':create' END;
            LET $view = IF $item.capabilities.view THEN RETURN $item.out.name.lowercase() + ':view' END;
            LET $edit = IF $item.capabilities.edit THEN RETURN $item.out.name.lowercase() + ':edit' END;
            LET $delete = IF $item.capabilities.delete THEN RETURN $item.out.name.lowercase() + ':delete' END;
            LET $arr = [$create, $view, $edit, $delete];
            $arr
        }).flatten().filter(|$item| type::is::string($item)) as permissions
FROM core_roles;