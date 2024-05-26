CREATE MIGRATION m1b3azt333w2src7kr75gnp7dqzmy2ncpk22skoud3zw6p5q2zpxfq
    ONTO m17ws5zbuvygq35txqmtnfci2si5s2cyjz7gv5xgly777jylleeppq
{
  ALTER TYPE default::Batch {
      ALTER PROPERTY purchased_quantity {
          DROP CONSTRAINT std::min_value(1);
      };
  };
  ALTER TYPE default::Batch {
      ALTER PROPERTY purchased_quantity {
          CREATE CONSTRAINT std::min_value(0);
      };
  };
  ALTER TYPE default::OrderLine {
      ALTER PROPERTY qty {
          DROP CONSTRAINT std::min_value(1);
      };
  };
  ALTER TYPE default::OrderLine {
      ALTER PROPERTY qty {
          CREATE CONSTRAINT std::min_value(0);
      };
      ALTER PROPERTY sku {
          CREATE CONSTRAINT std::max_len_value(255);
      };
  };
};
