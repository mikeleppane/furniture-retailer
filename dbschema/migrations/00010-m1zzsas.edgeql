CREATE MIGRATION m1zzsasysvph4bzlnvolbj6mbcbv2sipic6ajb52ihkxhwasgmxz3q
    ONTO m14izjzt5dvdrjqx34xyp3rnvofuzsu44h24zn47diag6n7ogk62ba
{
  ALTER TYPE default::Allocation {
      ALTER LINK batchid {
          SET REQUIRED USING (<default::Batch>{});
      };
      ALTER LINK orderid {
          SET REQUIRED USING (<default::OrderLine>{});
      };
  };
};
