window.BENCHMARK_DATA = {
  "lastUpdate": 1709771100617,
  "repoUrl": "https://github.com/fkz/lambda",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "development@schmitthenner.eu",
            "name": "Fabian Schmitthenner",
            "username": "fkz"
          },
          "committer": {
            "email": "development@schmitthenner.eu",
            "name": "Fabian Schmitthenner",
            "username": "fkz"
          },
          "distinct": true,
          "id": "3c3e30d8949e1c00f30ea9c265374134dc5e523e",
          "message": "experiment with smaller Program\n\nLet's see how the performance changes when we have smaller Program's,\nbut also more Box indirections",
          "timestamp": "2024-03-07T00:15:17Z",
          "tree_id": "1965d9c3f0623651a01c37f0784bac076cea3766",
          "url": "https://github.com/fkz/lambda/commit/3c3e30d8949e1c00f30ea9c265374134dc5e523e"
        },
        "date": 1709771100273,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "big-add/10000, 10000",
            "value": 4.161699856650433,
            "unit": "ms/op",
            "range": 0.0020395695623125223
          },
          {
            "name": "big-add/20000, 20000",
            "value": 7.841851909401454,
            "unit": "ms/op",
            "range": 0.003462540983855093
          },
          {
            "name": "big-add/30000, 30000",
            "value": 11.696138170977184,
            "unit": "ms/op",
            "range": 0.011796910243523988
          },
          {
            "name": "big-sub/   40,    20",
            "value": 0.23296186025982368,
            "unit": "ms/op",
            "range": 0.0001137355432573658
          },
          {
            "name": "big-sub/   80,    40",
            "value": 0.8455023924695033,
            "unit": "ms/op",
            "range": 0.0002651789654505125
          },
          {
            "name": "big-sub/  120,    60",
            "value": 1.8293710994849206,
            "unit": "ms/op",
            "range": 0.0006948993060074061
          },
          {
            "name": "small-add/   10,    10",
            "value": 0.0055661847905844036,
            "unit": "ms/op",
            "range": 0.0000034024384016417703
          },
          {
            "name": "small-add/   20,    20",
            "value": 0.010701338951169561,
            "unit": "ms/op",
            "range": 0.000008800006868363678
          },
          {
            "name": "small-add/   30,    30",
            "value": 0.014416793165674601,
            "unit": "ms/op",
            "range": 0.00003712727042668098
          },
          {
            "name": "small-add/   40,    40",
            "value": 0.018785054685748984,
            "unit": "ms/op",
            "range": 0.00002937529079525947
          },
          {
            "name": "small-add/   50,    50",
            "value": 0.024359899059667723,
            "unit": "ms/op",
            "range": 0.000017089041883270993
          },
          {
            "name": "small-add/   60,    60",
            "value": 0.026559438793203674,
            "unit": "ms/op",
            "range": 0.000011465381136526563
          },
          {
            "name": "small-add/   70,    70",
            "value": 0.03448167044831002,
            "unit": "ms/op",
            "range": 0.000026561624847776523
          },
          {
            "name": "small-add/   80,    80",
            "value": 0.03491815499991756,
            "unit": "ms/op",
            "range": 0.00001519285898948458
          },
          {
            "name": "small-add/   90,    90",
            "value": 0.04216371618752305,
            "unit": "ms/op",
            "range": 0.000035711132930326764
          },
          {
            "name": "small-add/  100,   100",
            "value": 0.04651678292712083,
            "unit": "ms/op",
            "range": 0.0002898975424715893
          },
          {
            "name": "small-sub/   20,    10",
            "value": 0.06279466146218979,
            "unit": "ms/op",
            "range": 0.0002328946939322543
          },
          {
            "name": "small-sub/   40,    20",
            "value": 0.22885510930935243,
            "unit": "ms/op",
            "range": 0.0001393037509058839
          },
          {
            "name": "small-sub/   60,    30",
            "value": 0.4851461084711254,
            "unit": "ms/op",
            "range": 0.0015990646663134331
          },
          {
            "name": "small-sub/   80,    40",
            "value": 0.8452983327034005,
            "unit": "ms/op",
            "range": 0.0030460628799029448
          },
          {
            "name": "small-sub/  100,    50",
            "value": 1.290313276318702,
            "unit": "ms/op",
            "range": 0.0017205385850677
          }
        ]
      }
    ]
  }
}