SELECT
    id.id() as id,
    name,
    custom,
    fn::metadata(metadata) as metadata
FROM core_permissions;