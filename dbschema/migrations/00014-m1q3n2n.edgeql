CREATE MIGRATION m1q3n2nmrmpjtoohhqtiinjs4nyfnfsbsewd7jyklul23a4jovc2ga
    ONTO m1zilobp75l6k7ots6gs2jh4qlzbet7r6srpwhnbonf3ankzowr5oa
{
  ALTER TYPE default::Batch {
      ALTER PROPERTY reference {
          CREATE CONSTRAINT std::min_len_value(1);
      };
      ALTER PROPERTY sku {
          CREATE CONSTRAINT std::min_len_value(1);
      };
  };
  ALTER TYPE default::OrderLine {
      ALTER PROPERTY orderid {
          DROP CONSTRAINT std::expression ON ((std::len(__subject__) > 0));
      };
  };
  ALTER TYPE default::OrderLine {
      ALTER PROPERTY orderid {
          CREATE CONSTRAINT std::min_len_value(1);
      };
      ALTER PROPERTY sku {
          CREATE CONSTRAINT std::min_len_value(1);
      };
  };
};
