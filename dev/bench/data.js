window.BENCHMARK_DATA = {
  "lastUpdate": 1709729805630,
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
          "id": "ee33b3dca6ba08fb2a88f385574eb3aa588f3214",
          "message": "fix once more",
          "timestamp": "2024-03-06T12:11:14Z",
          "tree_id": "b05f824c1fc20d694805f12a5e25906fcd760fac",
          "url": "https://github.com/fkz/lambda/commit/ee33b3dca6ba08fb2a88f385574eb3aa588f3214"
        },
        "date": 1709727617737,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "big-add/10000, 10000",
            "value": 6009830.49719246,
            "unit": "ns",
            "range": 3929.455665074327
          },
          {
            "name": "big-add/20000, 20000",
            "value": 11524557.70295635,
            "unit": "ns",
            "range": 24643.29136057128
          },
          {
            "name": "big-add/30000, 30000",
            "value": 17729202.134821426,
            "unit": "ns",
            "range": 15916.741873335714
          },
          {
            "name": "big-sub/   40,    20",
            "value": 285165.39194693236,
            "unit": "ns",
            "range": 156.57195932803108
          },
          {
            "name": "big-sub/   80,    40",
            "value": 1037258.928045184,
            "unit": "ns",
            "range": 427.6678265206047
          },
          {
            "name": "big-sub/  120,    60",
            "value": 2250230.064189896,
            "unit": "ns",
            "range": 1030.6821311273463
          },
          {
            "name": "small-add/   10,    10",
            "value": 6378.623501053168,
            "unit": "ns",
            "range": 18.485554528480968
          },
          {
            "name": "small-add/   20,    20",
            "value": 14383.203755376544,
            "unit": "ns",
            "range": 15.684648380283972
          },
          {
            "name": "small-add/   30,    30",
            "value": 20447.76158354865,
            "unit": "ns",
            "range": 17.713732439096336
          },
          {
            "name": "small-add/   40,    40",
            "value": 26748.633584395626,
            "unit": "ns",
            "range": 42.40552455034976
          },
          {
            "name": "small-add/   50,    50",
            "value": 33507.225534047815,
            "unit": "ns",
            "range": 41.548649146763125
          },
          {
            "name": "small-add/   60,    60",
            "value": 39473.80779847984,
            "unit": "ns",
            "range": 17.06171126048496
          },
          {
            "name": "small-add/   70,    70",
            "value": 46267.44471681993,
            "unit": "ns",
            "range": 72.99731247177147
          },
          {
            "name": "small-add/   80,    80",
            "value": 50520.7401335617,
            "unit": "ns",
            "range": 128.7948449895269
          },
          {
            "name": "small-add/   90,    90",
            "value": 55459.587545734954,
            "unit": "ns",
            "range": 39.4864931671325
          },
          {
            "name": "small-add/  100,   100",
            "value": 64828.487014211176,
            "unit": "ns",
            "range": 112.00607152339818
          },
          {
            "name": "small-sub/   20,    10",
            "value": 84010.14468859124,
            "unit": "ns",
            "range": 47.12823455730556
          },
          {
            "name": "small-sub/   40,    20",
            "value": 283851.02883442433,
            "unit": "ns",
            "range": 117.88293733713311
          },
          {
            "name": "small-sub/   60,    30",
            "value": 602562.9236088302,
            "unit": "ns",
            "range": 469.64852850742346
          },
          {
            "name": "small-sub/   80,    40",
            "value": 1036548.7330793324,
            "unit": "ns",
            "range": 3064.878314396729
          },
          {
            "name": "small-sub/  100,    50",
            "value": 1584726.4692557077,
            "unit": "ns",
            "range": 716.5882067455766
          }
        ]
      },
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
          "id": "dea1ea0be0bb77434fa138d0c0fa07054d9a1cf3",
          "message": "improve viewing: display ms instead of ns",
          "timestamp": "2024-03-06T12:47:28Z",
          "tree_id": "d4748db3977ada7b17bb19da4b6a651e63a1f58e",
          "url": "https://github.com/fkz/lambda/commit/dea1ea0be0bb77434fa138d0c0fa07054d9a1cf3"
        },
        "date": 1709729805242,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "big-add/10000, 10000",
            "value": 5966060543070.437,
            "unit": "ms/op",
            "range": 3380833574.4167223
          },
          {
            "name": "big-add/20000, 20000",
            "value": 11676525589404.762,
            "unit": "ms/op",
            "range": 5273267167.61773
          },
          {
            "name": "big-add/30000, 30000",
            "value": 17781589442480.156,
            "unit": "ms/op",
            "range": 66052563958.820015
          },
          {
            "name": "big-sub/   40,    20",
            "value": 283061231499.0634,
            "unit": "ms/op",
            "range": 46986303.363070615
          },
          {
            "name": "big-sub/   80,    40",
            "value": 1023315988027.0198,
            "unit": "ms/op",
            "range": 1035032459.3618456
          },
          {
            "name": "big-sub/  120,    60",
            "value": 2211236626378.496,
            "unit": "ms/op",
            "range": 586142002.6698853
          },
          {
            "name": "small-add/   10,    10",
            "value": 6390934253.106061,
            "unit": "ms/op",
            "range": 3936358.93882374
          },
          {
            "name": "small-add/   20,    20",
            "value": 14456531288.097443,
            "unit": "ms/op",
            "range": 20759884.909803253
          },
          {
            "name": "small-add/   30,    30",
            "value": 20855793231.688713,
            "unit": "ms/op",
            "range": 70945945.14358093
          },
          {
            "name": "small-add/   40,    40",
            "value": 27150962746.73355,
            "unit": "ms/op",
            "range": 35577077.35529918
          },
          {
            "name": "small-add/   50,    50",
            "value": 33529202396.71447,
            "unit": "ms/op",
            "range": 27367363.487481106
          },
          {
            "name": "small-add/   60,    60",
            "value": 39601608264.90983,
            "unit": "ms/op",
            "range": 21192260.641121704
          },
          {
            "name": "small-add/   70,    70",
            "value": 46328852770.52559,
            "unit": "ms/op",
            "range": 25519650.179328404
          },
          {
            "name": "small-add/   80,    80",
            "value": 52334888170.53116,
            "unit": "ms/op",
            "range": 47026609.204656385
          },
          {
            "name": "small-add/   90,    90",
            "value": 58109476977.59116,
            "unit": "ms/op",
            "range": 31444731.379309557
          },
          {
            "name": "small-add/  100,   100",
            "value": 65397464749.81791,
            "unit": "ms/op",
            "range": 86408578.56060752
          },
          {
            "name": "small-sub/   20,    10",
            "value": 84488526922.22337,
            "unit": "ms/op",
            "range": 44498507.00532644
          },
          {
            "name": "small-sub/   40,    20",
            "value": 285352489640.93195,
            "unit": "ms/op",
            "range": 512779196.9729817
          },
          {
            "name": "small-sub/   60,    30",
            "value": 596125181005.6279,
            "unit": "ms/op",
            "range": 271937840.03416425
          },
          {
            "name": "small-sub/   80,    40",
            "value": 1029356807894.959,
            "unit": "ms/op",
            "range": 1778018364.6258228
          },
          {
            "name": "small-sub/  100,    50",
            "value": 1562829443129.5078,
            "unit": "ms/op",
            "range": 1328398722.316771
          }
        ]
      }
    ]
  }
}