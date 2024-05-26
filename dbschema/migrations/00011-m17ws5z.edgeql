CREATE MIGRATION m17ws5zbuvygq35txqmtnfci2si5s2cyjz7gv5xgly777jylleeppq
    ONTO m1zzsasysvph4bzlnvolbj6mbcbv2sipic6ajb52ihkxhwasgmxz3q
{
  ALTER TYPE default::Batch {
      ALTER PROPERTY ref_ {
          RENAME TO reference;
      };
  };
};
