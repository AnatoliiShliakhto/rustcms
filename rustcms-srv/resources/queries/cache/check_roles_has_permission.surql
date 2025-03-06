BEGIN TRANSACTION;

LET $roles_recs = $roles.map(|$item| type::thing('cache_role_permissions', $item));
LET $permission_lowercase = $permission.lowercase();

RETURN $roles_recs.permissions.flatten().any($permission_lowercase);

COMMIT TRANSACTION;