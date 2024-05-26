CREATE MIGRATION m1hqwn4wpixsmggxudb4ledpxl5db6zmed3nhmcbywdmt5hmrp4u3a
    ONTO m14rcrj5uhmj4dyorxscjmlj7ays4jhxvfojp57jvc6bed7hy3kw4a
{
  ALTER TYPE default::Batch {
      ALTER PROPERTY sku {
          SET REQUIRED USING (<std::str>{});
      };
  };
  ALTER TYPE default::OrderLine {
      ALTER PROPERTY sku {
          SET REQUIRED USING (<std::str>{});
      };
  };
};
