CREATE MIGRATION m14izjzt5dvdrjqx34xyp3rnvofuzsu44h24zn47diag6n7ogk62ba
    ONTO m1mofiddqbhhzed2db4dx57lxxhx6lelf5am6loufyndicuxk5ahjq
{
  CREATE TYPE default::OrderLine {
      CREATE REQUIRED PROPERTY orderid: std::str {
          CREATE CONSTRAINT std::exclusive;
          CREATE CONSTRAINT std::max_len_value(255);
      };
      CREATE REQUIRED PROPERTY qty: std::int32 {
          CREATE CONSTRAINT std::min_value(1);
      };
      CREATE PROPERTY sku: std::str;
  };
  CREATE TYPE default::Allocation {
      CREATE MULTI LINK batchid: default::Batch;
      CREATE MULTI LINK orderid: default::OrderLine;
  };
  ALTER TYPE default::Batch {
      DROP PROPERTY allocations;
      ALTER PROPERTY purchased_quantity {
          CREATE CONSTRAINT std::min_value(1);
      };
      ALTER PROPERTY ref_ {
          CREATE CONSTRAINT std::exclusive;
          CREATE CONSTRAINT std::max_len_value(255);
      };
      ALTER PROPERTY sku {
          CREATE CONSTRAINT std::max_len_value(255);
          RESET OPTIONALITY;
      };
  };
};
