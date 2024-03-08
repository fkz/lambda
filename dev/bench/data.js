window.BENCHMARK_DATA = {
  "lastUpdate": 1709894755581,
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
          "id": "da1e45a82de2493587b044e9f3b0e77badbab643",
          "message": "Try out new \"garbage-collected\" program, to reduce malloc/free overhead",
          "timestamp": "2024-03-08T09:58:56Z",
          "tree_id": "9d4eb3c89ee6f0251ef5a39b6cd6b19f685a77ba",
          "url": "https://github.com/fkz/lambda/commit/da1e45a82de2493587b044e9f3b0e77badbab643"
        },
        "date": 1709892489726,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "big-add/10000, 10000",
            "value": 5.965113650240575,
            "unit": "ms/op",
            "range": 0.01227268485624112
          },
          {
            "name": "big-add/20000, 20000",
            "value": 11.88787954248512,
            "unit": "ms/op",
            "range": 0.018143890297459817
          },
          {
            "name": "big-add/30000, 30000",
            "value": 17.635210017255286,
            "unit": "ms/op",
            "range": 0.03225823312914116
          },
          {
            "name": "big-sub/   40,    20",
            "value": 0.03536898206166864,
            "unit": "ms/op",
            "range": 0.000004508411668752687
          },
          {
            "name": "big-sub/   80,    40",
            "value": 0.049271970477222864,
            "unit": "ms/op",
            "range": 0.000014731909213993951
          },
          {
            "name": "big-sub/  120,    60",
            "value": 0.06284824680684538,
            "unit": "ms/op",
            "range": 0.00001380947752964814
          },
          {
            "name": "small-add/   10,    10",
            "value": 0.007018520327832342,
            "unit": "ms/op",
            "range": 0.0000046429334643429165
          },
          {
            "name": "small-add/   20,    20",
            "value": 0.013468626307467587,
            "unit": "ms/op",
            "range": 0.000010240274984529559
          },
          {
            "name": "small-add/   30,    30",
            "value": 0.020608865110975063,
            "unit": "ms/op",
            "range": 0.00003064338724199883
          },
          {
            "name": "small-add/   40,    40",
            "value": 0.024880312099141048,
            "unit": "ms/op",
            "range": 0.000037603398090631435
          },
          {
            "name": "small-add/   50,    50",
            "value": 0.03403134644847513,
            "unit": "ms/op",
            "range": 0.00014203550432830792
          },
          {
            "name": "small-add/   60,    60",
            "value": 0.039636691744276635,
            "unit": "ms/op",
            "range": 0.000052045338427782936
          },
          {
            "name": "small-add/   70,    70",
            "value": 0.04293822735357254,
            "unit": "ms/op",
            "range": 0.000019115644481975314
          },
          {
            "name": "small-add/   80,    80",
            "value": 0.04826224793339192,
            "unit": "ms/op",
            "range": 0.000020988197608755432
          },
          {
            "name": "small-add/   90,    90",
            "value": 0.05822767291820608,
            "unit": "ms/op",
            "range": 0.0002932722945507051
          },
          {
            "name": "small-add/  100,   100",
            "value": 0.06330395973719914,
            "unit": "ms/op",
            "range": 0.00010413838106137039
          },
          {
            "name": "small-sub/   20,    10",
            "value": 0.009170353278405584,
            "unit": "ms/op",
            "range": 0.000013350223408364981
          },
          {
            "name": "small-sub/   40,    20",
            "value": 0.01647968203839294,
            "unit": "ms/op",
            "range": 0.000006519969031087169
          },
          {
            "name": "small-sub/   60,    30",
            "value": 0.023186914305749238,
            "unit": "ms/op",
            "range": 0.000009045257802138746
          },
          {
            "name": "small-sub/   80,    40",
            "value": 0.030222477808099776,
            "unit": "ms/op",
            "range": 0.0000225278151146227
          },
          {
            "name": "small-sub/  100,    50",
            "value": 0.036817721295015635,
            "unit": "ms/op",
            "range": 0.000025726602893155945
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
          "id": "1c5f0152d4814fed2b631c1074460cc72e880dbe",
          "message": "fix mutability bug",
          "timestamp": "2024-03-08T10:36:14Z",
          "tree_id": "5b1b50e0ee36afa95f451ba3de86e6ddb8550fa8",
          "url": "https://github.com/fkz/lambda/commit/1c5f0152d4814fed2b631c1074460cc72e880dbe"
        },
        "date": 1709894755224,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "big-add/10000, 10000",
            "value": 6.545653920504536,
            "unit": "ms/op",
            "range": 0.013205207040589991
          },
          {
            "name": "big-add/20000, 20000",
            "value": 13.235764850493199,
            "unit": "ms/op",
            "range": 0.02928441896025382
          },
          {
            "name": "big-add/30000, 30000",
            "value": 20.32526503788095,
            "unit": "ms/op",
            "range": 0.09186203204661504
          },
          {
            "name": "big-sub/   40,    20",
            "value": 6.539248676337868,
            "unit": "ms/op",
            "range": 0.002217801154354645
          },
          {
            "name": "big-sub/   80,    40",
            "value": 88.1425973363889,
            "unit": "ms/op",
            "range": 0.01992148102563082
          },
          {
            "name": "big-sub/  120,    60",
            "value": 424.52189005,
            "unit": "ms/op",
            "range": 0.6642936382879756
          },
          {
            "name": "small-add/   10,    10",
            "value": 0.008212220132774893,
            "unit": "ms/op",
            "range": 0.00001642626438809576
          },
          {
            "name": "small-add/   20,    20",
            "value": 0.01540157106666496,
            "unit": "ms/op",
            "range": 0.0000075702714335231294
          },
          {
            "name": "small-add/   30,    30",
            "value": 0.02065920461011506,
            "unit": "ms/op",
            "range": 0.00003539723677184946
          },
          {
            "name": "small-add/   40,    40",
            "value": 0.030677702862273448,
            "unit": "ms/op",
            "range": 0.000056186229244846746
          },
          {
            "name": "small-add/   50,    50",
            "value": 0.0335747023084394,
            "unit": "ms/op",
            "range": 0.0001804063816825229
          },
          {
            "name": "small-add/   60,    60",
            "value": 0.039251375660388124,
            "unit": "ms/op",
            "range": 0.000018297652668569805
          },
          {
            "name": "small-add/   70,    70",
            "value": 0.048582615096234336,
            "unit": "ms/op",
            "range": 0.00005343880979947708
          },
          {
            "name": "small-add/   80,    80",
            "value": 0.05186333669516174,
            "unit": "ms/op",
            "range": 0.000238849813672159
          },
          {
            "name": "small-add/   90,    90",
            "value": 0.05739161146756777,
            "unit": "ms/op",
            "range": 0.00002599106905282819
          },
          {
            "name": "small-add/  100,   100",
            "value": 0.06765835887439661,
            "unit": "ms/op",
            "range": 0.00021931102992762424
          },
          {
            "name": "small-sub/   20,    10",
            "value": 0.5693806991345279,
            "unit": "ms/op",
            "range": 0.0008515246045308185
          },
          {
            "name": "small-sub/   40,    20",
            "value": 6.52955818375,
            "unit": "ms/op",
            "range": 0.002451915135104704
          },
          {
            "name": "small-sub/   60,    30",
            "value": 29.39065626,
            "unit": "ms/op",
            "range": 0.003395500390302778
          },
          {
            "name": "small-sub/   80,    40",
            "value": 88.14788301,
            "unit": "ms/op",
            "range": 0.027783095726505183
          },
          {
            "name": "small-sub/  100,    50",
            "value": 208.82871128,
            "unit": "ms/op",
            "range": 0.31211146199554657
          }
        ]
      }
    ]
  }
}