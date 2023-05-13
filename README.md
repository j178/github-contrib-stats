# GitHub Contribution Stats

Generates a contribution summary for your GitHub profile. A Rust implementation of [yihong0618/github-readme-stats](https://github.com/yihong0618/github-readme-stats).

## Use it in GitHub action

```yml
name: Update README
on:
  schedule:
    - cron: '0 0 * * *'
jobs:
  update-readme:
    runs-on: ubuntu-latest
    name: Update README
    steps:
      - uses: actions/checkout@v4
      - name: Download github-contribution-stats
        uses: robinraju/release-downloader@v1.8
        with:
          repository: j178/github-contribution-stats
          latest: true
          fileName: "github-contribution-stats*.tar.gz"
          extract: true
      - name: Update stats
        run: ./github-contribution-stats -u ${{ github.repository_owner }} -o README.md
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      - name: Commit files
        run: |
            git config --global user.name github-actions
            git config --global user.email github-actions@github.com
            git add README.md
            git commit -m "Update README"
            git push
```

## Repos I Created

<!-- BEGIN:created_repos -->
| No.   | Name                                                                               | Language   | Stars | Forks | Last Update |
|-------|------------------------------------------------------------------------------------|------------|-------|-------|-------------|
| 1     | [leetgo](https://github.com/j178/leetgo)                                           | Go         | 380   | 18    | 2023-05-11  |
| 2     | [chatgpt](https://github.com/j178/chatgpt)                                         | Go         | 352   | 29    | 2023-04-29  |
| 3     | [fanfou-cli](https://github.com/j178/fanfou-cli)                                   | Python     | 22    | 5     | 2017-06-09  |
| 4     | [xiaoai-shutdown-my-computer](https://github.com/j178/xiaoai-shutdown-my-computer) | Python     | 17    | 0     | 2023-04-07  |
| 5     | [tiktoken-go](https://github.com/j178/tiktoken-go)                                 | Go         | 15    | 1     | 2023-04-21  |
| 6     | [ipgw](https://github.com/j178/ipgw)                                               | Python     | 9     | 0     | 2018-09-10  |
| 7     | [github-stargazer](https://github.com/j178/github-stargazer)                       | Go         | 8     | 0     | 2023-03-29  |
| 8     | [2022](https://github.com/j178/2022)                                               | Python     | 6     | 0     | 2022-12-31  |
| 9     | [spotlight](https://github.com/j178/spotlight)                                     | Python     | 4     | 0     | 2019-12-04  |
| 10    | [benchdiff](https://github.com/j178/benchdiff)                                     | Go         | 2     | 0     | 2022-10-30  |
| 11    | [j178](https://github.com/j178/j178)                                               | N/A        | 2     | 1     | 2023-05-12  |
| 12    | [twiyou](https://github.com/j178/twiyou)                                           | Go         | 2     | 0     | 2023-01-09  |
| 13    | [van](https://github.com/j178/van)                                                 | Python     | 2     | 1     | 2020-08-04  |
| 14    | [auto-snatch-course](https://github.com/j178/auto-snatch-course)                   | Python     | 1     | 0     | 2018-04-14  |
| 15    | [course-schedule-icalendar](https://github.com/j178/course-schedule-icalendar)     | Python     | 1     | 0     | 2017-02-10  |
| 16    | [fanfou-bots](https://github.com/j178/fanfou-bots)                                 | Python     | 1     | 1     | 2020-07-23  |
| 17    | [json-tutorial](https://github.com/j178/json-tutorial)                             | C          | 1     | 0     | 2017-01-19  |
| 18    | [leetcode](https://github.com/j178/leetcode)                                       | Go         | 1     | 0     | 2023-05-13  |
| 19    | [naive-sshd](https://github.com/j178/naive-sshd)                                   | Go         | 1     | 0     | 2019-10-15  |
| 20    | [neu6v-crawler](https://github.com/j178/neu6v-crawler)                             | Python     | 1     | 0     | 2016-10-29  |
| 21    | [scoop-bucket](https://github.com/j178/scoop-bucket)                               | PowerShell | 1     | 0     | 2023-05-08  |
| 22    | [v2ex](https://github.com/j178/v2ex)                                               | Python     | 1     | 0     | 2022-07-28  |
| Total |                                                                                    |            | 830   | 56    |             |
<!-- END:created_repos -->

## Repos I've Contributed To

<!-- BEGIN:contributed -->
| No.   | Name                                                                        | Stars | First PR                                                           | Last PR                                                            | PR Count |
|-------|-----------------------------------------------------------------------------|-------|--------------------------------------------------------------------|--------------------------------------------------------------------|----------|
| 1     | [encode/httpx](https://github.com/encode/httpx)                             | 0     | [2019-12-31](https://github.com/encode/httpx/pull/704)             | [2022-07-13](https://github.com/encode/httpx/pull/2302)            | 15       |
| 2     | [centrifugal/centrifuge](https://github.com/centrifugal/centrifuge)         | 0     | [2022-06-24](https://github.com/centrifugal/centrifuge/pull/230)   | [2022-09-09](https://github.com/centrifugal/centrifuge/pull/252)   | 10       |
| 3     | [encode/httpcore](https://github.com/encode/httpcore)                       | 0     | [2020-08-12](https://github.com/encode/httpcore/pull/154)          | [2022-07-13](https://github.com/encode/httpcore/pull/565)          | 8        |
| 4     | [disksing/twiyou](https://github.com/disksing/twiyou)                       | 0     | [2022-10-07](https://github.com/disksing/twiyou/pull/1)            | [2022-12-28](https://github.com/disksing/twiyou/pull/10)           | 7        |
| 5     | [redis/go-redis](https://github.com/redis/go-redis)                         | 0     | [2022-08-01](https://github.com/redis/go-redis/pull/2174)          | [2022-10-05](https://github.com/redis/go-redis/pull/2231)          | 4        |
| 6     | [centrifugal/centrifugo](https://github.com/centrifugal/centrifugo)         | 0     | [2022-07-18](https://github.com/centrifugal/centrifugo/pull/525)   | [2022-07-20](https://github.com/centrifugal/centrifugo/pull/528)   | 4        |
| 7     | [goreleaser/goreleaser](https://github.com/goreleaser/goreleaser)           | 0     | [2021-09-03](https://github.com/goreleaser/goreleaser/pull/2455)   | [2023-01-30](https://github.com/goreleaser/goreleaser/pull/3730)   | 3        |
| 8     | [zurawiki/tiktoken-rs](https://github.com/zurawiki/tiktoken-rs)             | 0     | [2023-04-03](https://github.com/zurawiki/tiktoken-rs/pull/14)      | [2023-04-04](https://github.com/zurawiki/tiktoken-rs/pull/15)      | 2        |
| 9     | [python/cpython](https://github.com/python/cpython)                         | 0     | [2021-06-16](https://github.com/python/cpython/pull/26754)         | [2021-12-22](https://github.com/python/cpython/pull/30227)         | 2        |
| 10    | [rq/rq](https://github.com/rq/rq)                                           | 0     | [2019-06-16](https://github.com/rq/rq/pull/1108)                   | [2019-06-16](https://github.com/rq/rq/pull/1109)                   | 2        |
| 11    | [caarlos0/fork-cleaner](https://github.com/caarlos0/fork-cleaner)           | 0     | [2023-05-11](https://github.com/caarlos0/fork-cleaner/pull/142)    | [2023-05-11](https://github.com/caarlos0/fork-cleaner/pull/142)    | 1        |
| 12    | [mitsuhiko/rye](https://github.com/mitsuhiko/rye)                           | 0     | [2023-05-10](https://github.com/mitsuhiko/rye/pull/127)            | [2023-05-10](https://github.com/mitsuhiko/rye/pull/127)            | 1        |
| 13    | [pkoukk/tiktoken-go](https://github.com/pkoukk/tiktoken-go)                 | 0     | [2023-04-08](https://github.com/pkoukk/tiktoken-go/pull/5)         | [2023-04-08](https://github.com/pkoukk/tiktoken-go/pull/5)         | 1        |
| 14    | [zurawiki/gptcommit](https://github.com/zurawiki/gptcommit)                 | 0     | [2023-04-06](https://github.com/zurawiki/gptcommit/pull/139)       | [2023-04-06](https://github.com/zurawiki/gptcommit/pull/139)       | 1        |
| 15    | [charmbracelet/bubbletea](https://github.com/charmbracelet/bubbletea)       | 0     | [2023-04-03](https://github.com/charmbracelet/bubbletea/pull/709)  | [2023-04-03](https://github.com/charmbracelet/bubbletea/pull/709)  | 1        |
| 16    | [sashabaranov/go-openai](https://github.com/sashabaranov/go-openai)         | 0     | [2023-03-20](https://github.com/sashabaranov/go-openai/pull/180)   | [2023-03-20](https://github.com/sashabaranov/go-openai/pull/180)   | 1        |
| 17    | [zellyn/kooky](https://github.com/zellyn/kooky)                             | 0     | [2023-01-05](https://github.com/zellyn/kooky/pull/56)              | [2023-01-05](https://github.com/zellyn/kooky/pull/56)              | 1        |
| 18    | [github/docs](https://github.com/github/docs)                               | 0     | [2022-11-10](https://github.com/github/docs/pull/21929)            | [2022-11-10](https://github.com/github/docs/pull/21929)            | 1        |
| 19    | [yihong0618/running_page](https://github.com/yihong0618/running_page)       | 0     | [2022-10-13](https://github.com/yihong0618/running_page/pull/319)  | [2022-10-13](https://github.com/yihong0618/running_page/pull/319)  | 1        |
| 20    | [pdm-project/pdm](https://github.com/pdm-project/pdm)                       | 0     | [2022-10-13](https://github.com/pdm-project/pdm/pull/1434)         | [2022-10-13](https://github.com/pdm-project/pdm/pull/1434)         | 1        |
| 21    | [redis/redis-doc](https://github.com/redis/redis-doc)                       | 0     | [2022-08-03](https://github.com/redis/redis-doc/pull/2064)         | [2022-08-03](https://github.com/redis/redis-doc/pull/2064)         | 1        |
| 22    | [Textualize/rich](https://github.com/Textualize/rich)                       | 0     | [2022-08-01](https://github.com/Textualize/rich/pull/2437)         | [2022-08-01](https://github.com/Textualize/rich/pull/2437)         | 1        |
| 23    | [centrifugal/centrifuge-go](https://github.com/centrifugal/centrifuge-go)   | 0     | [2022-06-29](https://github.com/centrifugal/centrifuge-go/pull/64) | [2022-06-29](https://github.com/centrifugal/centrifuge-go/pull/64) | 1        |
| 24    | [xbin-io/xbin](https://github.com/xbin-io/xbin)                             | 0     | [2022-04-25](https://github.com/xbin-io/xbin/pull/2)               | [2022-04-25](https://github.com/xbin-io/xbin/pull/2)               | 1        |
| 25    | [EndlessCheng/codeforces-go](https://github.com/EndlessCheng/codeforces-go) | 0     | [2022-04-22](https://github.com/EndlessCheng/codeforces-go/pull/3) | [2022-04-22](https://github.com/EndlessCheng/codeforces-go/pull/3) | 1        |
| 26    | [golang/go](https://github.com/golang/go)                                   | 0     | [2022-04-06](https://github.com/golang/go/pull/52194)              | [2022-04-06](https://github.com/golang/go/pull/52194)              | 1        |
| 27    | [yihong0618/GitHubPoster](https://github.com/yihong0618/GitHubPoster)       | 0     | [2022-02-15](https://github.com/yihong0618/GitHubPoster/pull/56)   | [2022-02-15](https://github.com/yihong0618/GitHubPoster/pull/56)   | 1        |
| 28    | [gofiber/fiber](https://github.com/gofiber/fiber)                           | 0     | [2021-08-30](https://github.com/gofiber/fiber/pull/1510)           | [2021-08-30](https://github.com/gofiber/fiber/pull/1510)           | 1        |
| 29    | [pydantic/pydantic](https://github.com/pydantic/pydantic)                   | 0     | [2021-03-25](https://github.com/pydantic/pydantic/pull/2577)       | [2021-03-25](https://github.com/pydantic/pydantic/pull/2577)       | 1        |
| 30    | [urllib3/urllib3](https://github.com/urllib3/urllib3)                       | 0     | [2020-11-26](https://github.com/urllib3/urllib3/pull/2095)         | [2020-11-26](https://github.com/urllib3/urllib3/pull/2095)         | 1        |
| 31    | [aio-libs/yarl](https://github.com/aio-libs/yarl)                           | 0     | [2020-05-20](https://github.com/aio-libs/yarl/pull/452)            | [2020-05-20](https://github.com/aio-libs/yarl/pull/452)            | 1        |
| 32    | [7sDream/kd100](https://github.com/7sDream/kd100)                           | 0     | [2016-11-16](https://github.com/7sDream/kd100/pull/1)              | [2016-11-16](https://github.com/7sDream/kd100/pull/1)              | 1        |
| Total |                                                                             |       |                                                                    |                                                                    | 79       |
<!-- END:contributed -->
