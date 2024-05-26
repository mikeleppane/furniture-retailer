CREATE MIGRATION m1mofiddqbhhzed2db4dx57lxxhx6lelf5am6loufyndicuxk5ahjq
    ONTO m1fkp7booazjt7pojucabtiuuk43ootwqlvycafneowc3k5nrfv4ma
{
  ALTER TYPE default::Batch {
      ALTER PROPERTY eta {
          SET TYPE cal::local_date USING (<cal::local_date>.eta);
      };
  };
};
