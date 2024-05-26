CREATE MIGRATION m1zilobp75l6k7ots6gs2jh4qlzbet7r6srpwhnbonf3ankzowr5oa
    ONTO m1b3azt333w2src7kr75gnp7dqzmy2ncpk22skoud3zw6p5q2zpxfq
{
  ALTER TYPE default::OrderLine {
      ALTER PROPERTY orderid {
          CREATE CONSTRAINT std::expression ON ((std::len(__subject__) > 0));
      };
  };
};
