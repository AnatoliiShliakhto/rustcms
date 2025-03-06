BEGIN TRANSACTION;

DELETE FROM cache_role_permissions;

FOR $role_rec IN $roles_with_permissions {
    CREATE cache_role_permissions CONTENT $role_rec;
};

COMMIT TRANSACTION;