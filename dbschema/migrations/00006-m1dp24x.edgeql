CREATE MIGRATION m1dp24x4habuuacq5ndxzh7xzsqy45u7wcydfvgfg6jol34qq7tzvq
    ONTO m1vravofqa3wkiaptkqvs7eeoyg275hk3qx4db77csi7jr5lttyltq
{
  ALTER TYPE default::Batch {
      ALTER PROPERTY eta {
          SET TYPE cal::local_time USING (<cal::local_time>.eta);
      };
  };
};
