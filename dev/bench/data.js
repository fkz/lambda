window.BENCHMARK_DATA = {
  "lastUpdate": 1709908177735,
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
          "id": "f0b582afab1b93452efb6714589436cc9cbd6fbb",
          "message": "Revert \"experiment with smaller Program\"\n\nThis reverts commit 3c3e30d8949e1c00f30ea9c265374134dc5e523e.",
          "timestamp": "2024-03-07T00:27:42Z",
          "tree_id": "3f78cc73ca10a1862e875239be91243164917986",
          "url": "https://github.com/fkz/lambda/commit/f0b582afab1b93452efb6714589436cc9cbd6fbb"
        },
        "date": 1709771814655,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "big-add/10000, 10000",
            "value": 6.053959939888889,
            "unit": "ms/op",
            "range": 0.007752199423442843
          },
          {
            "name": "big-add/20000, 20000",
            "value": 11.7261978600248,
            "unit": "ms/op",
            "range": 0.10911827028110771
          },
          {
            "name": "big-add/30000, 30000",
            "value": 18.359743736936508,
            "unit": "ms/op",
            "range": 0.045364091670153275
          },
          {
            "name": "big-sub/   40,    20",
            "value": 0.28945674364222723,
            "unit": "ms/op",
            "range": 0.0007523292015192135
          },
          {
            "name": "big-sub/   80,    40",
            "value": 1.043623186046176,
            "unit": "ms/op",
            "range": 0.0004022578475818187
          },
          {
            "name": "big-sub/  120,    60",
            "value": 2.260359963507549,
            "unit": "ms/op",
            "range": 0.0007816015922895387
          },
          {
            "name": "small-add/   10,    10",
            "value": 0.006394520749841297,
            "unit": "ms/op",
            "range": 0.000005650610804607882
          },
          {
            "name": "small-add/   20,    20",
            "value": 0.014478924087793806,
            "unit": "ms/op",
            "range": 0.00003342371117839286
          },
          {
            "name": "small-add/   30,    30",
            "value": 0.020876580443887804,
            "unit": "ms/op",
            "range": 0.00004668695193811959
          },
          {
            "name": "small-add/   40,    40",
            "value": 0.026513874401713787,
            "unit": "ms/op",
            "range": 0.000015201202770048511
          },
          {
            "name": "small-add/   50,    50",
            "value": 0.03354246398686261,
            "unit": "ms/op",
            "range": 0.0000250779229213711
          },
          {
            "name": "small-add/   60,    60",
            "value": 0.04015911194760912,
            "unit": "ms/op",
            "range": 0.00005750191698717291
          },
          {
            "name": "small-add/   70,    70",
            "value": 0.04467140621053937,
            "unit": "ms/op",
            "range": 0.000056333182927451796
          },
          {
            "name": "small-add/   80,    80",
            "value": 0.05298650287634727,
            "unit": "ms/op",
            "range": 0.00010474774384787048
          },
          {
            "name": "small-add/   90,    90",
            "value": 0.05889742148918776,
            "unit": "ms/op",
            "range": 0.00013567434973201973
          },
          {
            "name": "small-add/  100,   100",
            "value": 0.06513868204496776,
            "unit": "ms/op",
            "range": 0.00003602784743400674
          },
          {
            "name": "small-sub/   20,    10",
            "value": 0.085757499748425,
            "unit": "ms/op",
            "range": 0.00004215225741495655
          },
          {
            "name": "small-sub/   40,    20",
            "value": 0.28972348096101314,
            "unit": "ms/op",
            "range": 0.00025207331946038584
          },
          {
            "name": "small-sub/   60,    30",
            "value": 0.6088368245100596,
            "unit": "ms/op",
            "range": 0.0003629094751382382
          },
          {
            "name": "small-sub/   80,    40",
            "value": 1.0464897665672217,
            "unit": "ms/op",
            "range": 0.0009307132160694211
          },
          {
            "name": "small-sub/  100,    50",
            "value": 1.6145175351071666,
            "unit": "ms/op",
            "range": 0.008446670815946486
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
          "id": "ac2c86a480562c85157ee137df4b39e2f5d417f0",
          "message": "Revert \"Revert \"experiment with smaller Program\"\"\n\nThis reverts commit f0b582afab1b93452efb6714589436cc9cbd6fbb.",
          "timestamp": "2024-03-07T09:22:23Z",
          "tree_id": "1965d9c3f0623651a01c37f0784bac076cea3766",
          "url": "https://github.com/fkz/lambda/commit/ac2c86a480562c85157ee137df4b39e2f5d417f0"
        },
        "date": 1709803887192,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "big-add/10000, 10000",
            "value": 4.212068795766594,
            "unit": "ms/op",
            "range": 0.003279841063375524
          },
          {
            "name": "big-add/20000, 20000",
            "value": 7.930564294659389,
            "unit": "ms/op",
            "range": 0.0058972393728913995
          },
          {
            "name": "big-add/30000, 30000",
            "value": 11.797465776974208,
            "unit": "ms/op",
            "range": 0.01154323631322374
          },
          {
            "name": "big-sub/   40,    20",
            "value": 0.23175076241185022,
            "unit": "ms/op",
            "range": 0.00007819528058720326
          },
          {
            "name": "big-sub/   80,    40",
            "value": 0.8383930777082423,
            "unit": "ms/op",
            "range": 0.00028055390975601245
          },
          {
            "name": "big-sub/  120,    60",
            "value": 1.8282388611825395,
            "unit": "ms/op",
            "range": 0.0015034443595508642
          },
          {
            "name": "small-add/   10,    10",
            "value": 0.00553878860583074,
            "unit": "ms/op",
            "range": 0.000002640991713084007
          },
          {
            "name": "small-add/   20,    20",
            "value": 0.01095318395913055,
            "unit": "ms/op",
            "range": 0.00003297441580486505
          },
          {
            "name": "small-add/   30,    30",
            "value": 0.013722419808686251,
            "unit": "ms/op",
            "range": 0.00002905733840473059
          },
          {
            "name": "small-add/   40,    40",
            "value": 0.017573091892143233,
            "unit": "ms/op",
            "range": 0.000013911178200600795
          },
          {
            "name": "small-add/   50,    50",
            "value": 0.023346030314678336,
            "unit": "ms/op",
            "range": 0.000015985587934652572
          },
          {
            "name": "small-add/   60,    60",
            "value": 0.027235869618944097,
            "unit": "ms/op",
            "range": 0.00003927227372258136
          },
          {
            "name": "small-add/   70,    70",
            "value": 0.028119421699714483,
            "unit": "ms/op",
            "range": 0.000040632302734705434
          },
          {
            "name": "small-add/   80,    80",
            "value": 0.03778811123580599,
            "unit": "ms/op",
            "range": 0.000024080502012577582
          },
          {
            "name": "small-add/   90,    90",
            "value": 0.03600437914516342,
            "unit": "ms/op",
            "range": 0.000055969049504928164
          },
          {
            "name": "small-add/  100,   100",
            "value": 0.04653740972013026,
            "unit": "ms/op",
            "range": 0.000021513372425664776
          },
          {
            "name": "small-sub/   20,    10",
            "value": 0.06557970931748064,
            "unit": "ms/op",
            "range": 0.00008295995698819494
          },
          {
            "name": "small-sub/   40,    20",
            "value": 0.23154779435041023,
            "unit": "ms/op",
            "range": 0.00024557341642461323
          },
          {
            "name": "small-sub/   60,    30",
            "value": 0.48451245150037703,
            "unit": "ms/op",
            "range": 0.00018904473385891716
          },
          {
            "name": "small-sub/   80,    40",
            "value": 0.8387979072756688,
            "unit": "ms/op",
            "range": 0.0006807481831783541
          },
          {
            "name": "small-sub/  100,    50",
            "value": 1.281875279273175,
            "unit": "ms/op",
            "range": 0.001467591913811894
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
          "id": "ed91f3d56e679c67487ff708849a97ff813f42d6",
          "message": "Revert \"does garbage collection help?\"\n\nThis reverts commit 4d067b008bdf5b9631e4ea29c187b6bd317d8247.",
          "timestamp": "2024-03-08T12:23:10Z",
          "tree_id": "e4e74db1bc211a2e01a81994b01be76f71d2ae7e",
          "url": "https://github.com/fkz/lambda/commit/ed91f3d56e679c67487ff708849a97ff813f42d6"
        },
        "date": 1709901141823,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "big-add/10000, 10000",
            "value": 6.286427266777778,
            "unit": "ms/op",
            "range": 0.006468139376789597
          },
          {
            "name": "big-add/20000, 20000",
            "value": 12.543340709627977,
            "unit": "ms/op",
            "range": 0.010067353577447013
          },
          {
            "name": "big-add/30000, 30000",
            "value": 18.86151819663492,
            "unit": "ms/op",
            "range": 0.03545808638202558
          },
          {
            "name": "big-sub/   40,    20",
            "value": 0.3003197568925821,
            "unit": "ms/op",
            "range": 0.00014318326147368435
          },
          {
            "name": "big-sub/   80,    40",
            "value": 1.0646664329891105,
            "unit": "ms/op",
            "range": 0.0003771207250532817
          },
          {
            "name": "big-sub/  120,    60",
            "value": 2.298602282350198,
            "unit": "ms/op",
            "range": 0.0004202390095574066
          },
          {
            "name": "small-add/   10,    10",
            "value": 0.007707290675996508,
            "unit": "ms/op",
            "range": 0.000005496344683054254
          },
          {
            "name": "small-add/   20,    20",
            "value": 0.015487940970731143,
            "unit": "ms/op",
            "range": 0.000015855388184606367
          },
          {
            "name": "small-add/   30,    30",
            "value": 0.019388407668159666,
            "unit": "ms/op",
            "range": 0.000018587900805112428
          },
          {
            "name": "small-add/   40,    40",
            "value": 0.029294835124286704,
            "unit": "ms/op",
            "range": 0.000018164011680868787
          },
          {
            "name": "small-add/   50,    50",
            "value": 0.03127962179009911,
            "unit": "ms/op",
            "range": 0.000017471301607070034
          },
          {
            "name": "small-add/   60,    60",
            "value": 0.03666811707702959,
            "unit": "ms/op",
            "range": 0.000017448273175370294
          },
          {
            "name": "small-add/   70,    70",
            "value": 0.04606291690062064,
            "unit": "ms/op",
            "range": 0.00004288955285256828
          },
          {
            "name": "small-add/   80,    80",
            "value": 0.055148540950553486,
            "unit": "ms/op",
            "range": 0.00007139727698079031
          },
          {
            "name": "small-add/   90,    90",
            "value": 0.06208344671280685,
            "unit": "ms/op",
            "range": 0.00003066320508632993
          },
          {
            "name": "small-add/  100,   100",
            "value": 0.0677697407117765,
            "unit": "ms/op",
            "range": 0.00019736799746559623
          },
          {
            "name": "small-sub/   20,    10",
            "value": 0.0802987299544495,
            "unit": "ms/op",
            "range": 0.00006977635922668218
          },
          {
            "name": "small-sub/   40,    20",
            "value": 0.2799864359275332,
            "unit": "ms/op",
            "range": 0.00016508774787291013
          },
          {
            "name": "small-sub/   60,    30",
            "value": 0.6035890672243407,
            "unit": "ms/op",
            "range": 0.000855732311587011
          },
          {
            "name": "small-sub/   80,    40",
            "value": 1.042587466232363,
            "unit": "ms/op",
            "range": 0.0014120967994946927
          },
          {
            "name": "small-sub/  100,    50",
            "value": 1.604106122972206,
            "unit": "ms/op",
            "range": 0.0007961829446523253
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
          "id": "0ed93d7bbadfe402ae9783d66e982628bfccf1b5",
          "message": "do copying when replacing",
          "timestamp": "2024-03-08T14:20:43Z",
          "tree_id": "8ff9116c4cabdf5f036d62f20e9ab3ef91f1da78",
          "url": "https://github.com/fkz/lambda/commit/0ed93d7bbadfe402ae9783d66e982628bfccf1b5"
        },
        "date": 1709908177338,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "big-add/10000, 10000",
            "value": 5.898980239875993,
            "unit": "ms/op",
            "range": 0.007546319402367004
          },
          {
            "name": "big-add/20000, 20000",
            "value": 11.860994603680556,
            "unit": "ms/op",
            "range": 0.01621632380292063
          },
          {
            "name": "big-add/30000, 30000",
            "value": 17.68031332382936,
            "unit": "ms/op",
            "range": 0.021650487624226753
          },
          {
            "name": "big-sub/   40,    20",
            "value": 0.21994695757936508,
            "unit": "ms/op",
            "range": 0.00007458918354214567
          },
          {
            "name": "big-sub/   80,    40",
            "value": 0.7541133274773711,
            "unit": "ms/op",
            "range": 0.0002569160022766513
          },
          {
            "name": "big-sub/  120,    60",
            "value": 1.6217182251663884,
            "unit": "ms/op",
            "range": 0.00038808001511145057
          },
          {
            "name": "small-add/   10,    10",
            "value": 0.007068779335833983,
            "unit": "ms/op",
            "range": 0.000007140380307531894
          },
          {
            "name": "small-add/   20,    20",
            "value": 0.014586862737858504,
            "unit": "ms/op",
            "range": 0.000007538690674610337
          },
          {
            "name": "small-add/   30,    30",
            "value": 0.01808271919013081,
            "unit": "ms/op",
            "range": 0.00002606338056164091
          },
          {
            "name": "small-add/   40,    40",
            "value": 0.027155311187331825,
            "unit": "ms/op",
            "range": 0.00002525198728036935
          },
          {
            "name": "small-add/   50,    50",
            "value": 0.03326094867772852,
            "unit": "ms/op",
            "range": 0.000021381290791289405
          },
          {
            "name": "small-add/   60,    60",
            "value": 0.03444693390735474,
            "unit": "ms/op",
            "range": 0.000040372105643106465
          },
          {
            "name": "small-add/   70,    70",
            "value": 0.03989929829776584,
            "unit": "ms/op",
            "range": 0.00003052109050840804
          },
          {
            "name": "small-add/   80,    80",
            "value": 0.0492653159460296,
            "unit": "ms/op",
            "range": 0.0001769725374683011
          },
          {
            "name": "small-add/   90,    90",
            "value": 0.054145845334585885,
            "unit": "ms/op",
            "range": 0.00007789111916753511
          },
          {
            "name": "small-add/  100,   100",
            "value": 0.060441995693775535,
            "unit": "ms/op",
            "range": 0.000018735284166591163
          },
          {
            "name": "small-sub/   20,    10",
            "value": 0.05901359210358307,
            "unit": "ms/op",
            "range": 0.0000631220539626537
          },
          {
            "name": "small-sub/   40,    20",
            "value": 0.20127214554058293,
            "unit": "ms/op",
            "range": 0.00006530302828278136
          },
          {
            "name": "small-sub/   60,    30",
            "value": 0.4260647929825519,
            "unit": "ms/op",
            "range": 0.0001940738842875301
          },
          {
            "name": "small-sub/   80,    40",
            "value": 0.7358939605153905,
            "unit": "ms/op",
            "range": 0.0017487673475034661
          },
          {
            "name": "small-sub/  100,    50",
            "value": 1.1317893062778397,
            "unit": "ms/op",
            "range": 0.0009970154357434831
          }
        ]
      }
    ]
  }
}