CREATE MIGRATION m1fkp7booazjt7pojucabtiuuk43ootwqlvycafneowc3k5nrfv4ma
    ONTO m1dp24x4habuuacq5ndxzh7xzsqy45u7wcydfvgfg6jol34qq7tzvq
{
  ALTER TYPE default::Batch {
      ALTER PROPERTY eta {
          SET TYPE std::str USING (<std::str>.eta);
      };
  };
};
