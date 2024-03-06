window.BENCHMARK_DATA = {
  "lastUpdate": 1709733708200,
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
            "value": 6.00983049719246,
            "unit": "ns",
            "range": 3929.455665074327
          },
          {
            "name": "big-add/20000, 20000",
            "value": 11.52455770295635,
            "unit": "ns",
            "range": 24643.29136057128
          },
          {
            "name": "big-add/30000, 30000",
            "value": 17.729202134821426,
            "unit": "ns",
            "range": 15916.741873335714
          },
          {
            "name": "big-sub/   40,    20",
            "value": .28516539194693236,
            "unit": "ns",
            "range": 156.57195932803108
          },
          {
            "name": "big-sub/   80,    40",
            "value": 1.037258928045184,
            "unit": "ns",
            "range": 427.6678265206047
          },
          {
            "name": "big-sub/  120,    60",
            "value": 2.250230064189896,
            "unit": "ns",
            "range": 1030.6821311273463
          },
          {
            "name": "small-add/   10,    10",
            "value": 0.006378623501053168,
            "unit": "ns",
            "range": 18.485554528480968
          },
          {
            "name": "small-add/   20,    20",
            "value": 0.014383203755376544,
            "unit": "ns",
            "range": 15.684648380283972
          },
          {
            "name": "small-add/   30,    30",
            "value": 0.02044776158354865,
            "unit": "ns",
            "range": 17.713732439096336
          },
          {
            "name": "small-add/   40,    40",
            "value": 0.026748633584395626,
            "unit": "ns",
            "range": 42.40552455034976
          },
          {
            "name": "small-add/   50,    50",
            "value": 0.033507225534047815,
            "unit": "ns",
            "range": 41.548649146763125
          },
          {
            "name": "small-add/   60,    60",
            "value": 0.03947380779847984,
            "unit": "ns",
            "range": 17.06171126048496
          },
          {
            "name": "small-add/   70,    70",
            "value": 0.04626744471681993,
            "unit": "ns",
            "range": 72.99731247177147
          },
          {
            "name": "small-add/   80,    80",
            "value": 0.0505207401335617,
            "unit": "ns",
            "range": 128.7948449895269
          },
          {
            "name": "small-add/   90,    90",
            "value": 0.055459587545734954,
            "unit": "ns",
            "range": 39.4864931671325
          },
          {
            "name": "small-add/  100,   100",
            "value": 0.064828487014211176,
            "unit": "ns",
            "range": 112.00607152339818
          },
          {
            "name": "small-sub/   20,    10",
            "value": 0.08401014468859124,
            "unit": "ns",
            "range": 47.12823455730556
          },
          {
            "name": "small-sub/   40,    20",
            "value": 0.28385102883442433,
            "unit": "ns",
            "range": 117.88293733713311
          },
          {
            "name": "small-sub/   60,    30",
            "value": 0.6025629236088302,
            "unit": "ns",
            "range": 469.64852850742346
          },
          {
            "name": "small-sub/   80,    40",
            "value": 1.0365487330793324,
            "unit": "ns",
            "range": 3064.878314396729
          },
          {
            "name": "small-sub/  100,    50",
            "value": 1.5847264692557077,
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
            "value": 5.966060543070437,
            "unit": "ms/op",
            "range": 3380833574.4167223
          },
          {
            "name": "big-add/20000, 20000",
            "value": 11.676525589404762,
            "unit": "ms/op",
            "range": 5273267167.61773
          },
          {
            "name": "big-add/30000, 30000",
            "value": 17.781589442480156,
            "unit": "ms/op",
            "range":0.066052563958820015
          },
          {
            "name": "big-sub/   40,    20",
            "value": .2830612314990634,
            "unit": "ms/op",
            "range": 46986303.363070615
          },
          {
            "name": "big-sub/   80,    40",
            "value": 1.0233159880270198,
            "unit": "ms/op",
            "range": 1035032459.3618456
          },
          {
            "name": "big-sub/  120,    60",
            "value": 2.211236626378496,
            "unit": "ms/op",
            "range": 586142002.6698853
          },
          {
            "name": "small-add/   10,    10",
            "value": 0.006390934253106061,
            "unit": "ms/op",
            "range": 3936358.93882374
          },
          {
            "name": "small-add/   20,    20",
            "value":0.014456531288097443,
            "unit": "ms/op",
            "range": 20759884.909803253
          },
          {
            "name": "small-add/   30,    30",
            "value":0.020855793231688713,
            "unit": "ms/op",
            "range": 70945945.14358093
          },
          {
            "name": "small-add/   40,    40",
            "value":0.02715096274673355,
            "unit": "ms/op",
            "range": 35577077.35529918
          },
          {
            "name": "small-add/   50,    50",
            "value":0.03352920239671447,
            "unit": "ms/op",
            "range": 27367363.487481106
          },
          {
            "name": "small-add/   60,    60",
            "value":0.03960160826490983,
            "unit": "ms/op",
            "range": 21192260.641121704
          },
          {
            "name": "small-add/   70,    70",
            "value":0.04632885277052559,
            "unit": "ms/op",
            "range": 25519650.179328404
          },
          {
            "name": "small-add/   80,    80",
            "value":0.05233488817053116,
            "unit": "ms/op",
            "range": 47026609.204656385
          },
          {
            "name": "small-add/   90,    90",
            "value":0.05810947697759116,
            "unit": "ms/op",
            "range": 31444731.379309557
          },
          {
            "name": "small-add/  100,   100",
            "value":0.06539746474981791,
            "unit": "ms/op",
            "range": 86408578.56060752
          },
          {
            "name": "small-sub/   20,    10",
            "value":0.08448852692222337,
            "unit": "ms/op",
            "range": 44498507.00532644
          },
          {
            "name": "small-sub/   40,    20",
            "value": .28535248964093195,
            "unit": "ms/op",
            "range": 512779196.9729817
          },
          {
            "name": "small-sub/   60,    30",
            "value": .5961251810056279,
            "unit": "ms/op",
            "range": 271937840.03416425
          },
          {
            "name": "small-sub/   80,    40",
            "value": 1.029356807894959,
            "unit": "ms/op",
            "range": 1778018364.6258228
          },
          {
            "name": "small-sub/  100,    50",
            "value": 1.5628294431295078,
            "unit": "ms/op",
            "range": 1328398722.316771
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
          "id": "f5e33ed05d6f0e185ce5c27bc7db1c60ecd57782",
          "message": "wrong order :see_no_evil:",
          "timestamp": "2024-03-06T13:52:39Z",
          "tree_id": "3f78cc73ca10a1862e875239be91243164917986",
          "url": "https://github.com/fkz/lambda/commit/f5e33ed05d6f0e185ce5c27bc7db1c60ecd57782"
        },
        "date": 1709733707784,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "big-add/10000, 10000",
            "value": 5.995721236845238,
            "unit": "ms/op",
            "range": 0.007641579723183622
          },
          {
            "name": "big-add/20000, 20000",
            "value": 11.726094782266864,
            "unit": "ms/op",
            "range": 0.03210623653866376
          },
          {
            "name": "big-add/30000, 30000",
            "value": 17.755263300092594,
            "unit": "ms/op",
            "range": 0.011462114906999955
          },
          {
            "name": "big-sub/   40,    20",
            "value": 0.2860759090380405,
            "unit": "ms/op",
            "range": 0.00017149358649932447
          },
          {
            "name": "big-sub/   80,    40",
            "value": 1.0250791632575351,
            "unit": "ms/op",
            "range": 0.0005250170766872715
          },
          {
            "name": "big-sub/  120,    60",
            "value": 2.2221391638265304,
            "unit": "ms/op",
            "range": 0.0032884248928079893
          },
          {
            "name": "small-add/   10,    10",
            "value": 0.00636423324875632,
            "unit": "ms/op",
            "range": 0.00001037992156018867
          },
          {
            "name": "small-add/   20,    20",
            "value": 0.014239593856242893,
            "unit": "ms/op",
            "range": 0.0000068425847044999
          },
          {
            "name": "small-add/   30,    30",
            "value": 0.02049471664817901,
            "unit": "ms/op",
            "range": 0.000019483714488852234
          },
          {
            "name": "small-add/   40,    40",
            "value": 0.02566458568315782,
            "unit": "ms/op",
            "range": 0.000017767270946521076
          },
          {
            "name": "small-add/   50,    50",
            "value": 0.03350572792182174,
            "unit": "ms/op",
            "range": 0.000018834385937682016
          },
          {
            "name": "small-add/   60,    60",
            "value": 0.039570679393019864,
            "unit": "ms/op",
            "range": 0.00006254421892004963
          },
          {
            "name": "small-add/   70,    70",
            "value": 0.046583684223270784,
            "unit": "ms/op",
            "range": 0.00007673033276198155
          },
          {
            "name": "small-add/   80,    80",
            "value": 0.05291070978518312,
            "unit": "ms/op",
            "range": 0.00016604084002492053
          },
          {
            "name": "small-add/   90,    90",
            "value": 0.05665817652537124,
            "unit": "ms/op",
            "range": 0.00022257351240530604
          },
          {
            "name": "small-add/  100,   100",
            "value": 0.0640646922192826,
            "unit": "ms/op",
            "range": 0.00005717762019267155
          },
          {
            "name": "small-sub/   20,    10",
            "value": 0.08602158539690619,
            "unit": "ms/op",
            "range": 0.0000664676878816574
          },
          {
            "name": "small-sub/   40,    20",
            "value": 0.2863203118833486,
            "unit": "ms/op",
            "range": 0.0004130291131838692
          },
          {
            "name": "small-sub/   60,    30",
            "value": 0.5994717989394696,
            "unit": "ms/op",
            "range": 0.0007967674220448252
          },
          {
            "name": "small-sub/   80,    40",
            "value": 1.0312640755126616,
            "unit": "ms/op",
            "range": 0.003939027034696043
          },
          {
            "name": "small-sub/  100,    50",
            "value": 1.5692553946468741,
            "unit": "ms/op",
            "range": 0.0009701899755486252
          }
        ]
      }
    ]
  }
}