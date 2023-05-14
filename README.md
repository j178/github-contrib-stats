# GitHub Contribution Stats

Generates a contribution summary for your GitHub profile. A Rust implementation of [yihong0618/github-readme-stats](https://github.com/yihong0618/github-readme-stats).

## Use it in GitHub action

```yml
name: Update README
on:
  workflow_dispatch:
  schedule:
    - cron: '0 0 * * *'
permissions:
  contents: write
jobs:
  update-readme:
    runs-on: ubuntu-latest
    name: Update README
    steps:
      - uses: actions/checkout@v3
      - name: Download github-contrib-stats
        uses: robinraju/release-downloader@v1.8
        with:
          repository: j178/github-contrib-stats
          latest: true
          fileName: "github-contrib-stats*.tar.gz"
          extract: true
      - name: Update stats
        run: ./github-contrib-stats -u ${{ github.repository_owner }} -o README.md
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      - name: Commit files
        run: |
          if git diff --quiet; then
            echo "nothing new"
            exit 0
          fi
          git config user.name github-actions
          git config user.email github-actions@github.com
          git add README.md
          git commit -m "Update README"
          git push
```

## Repos I Created

<!-- BEGIN:created_repos -->
| No.   | Name                                                                               | Language   | Stars | Forks | Last Update |
|-------|------------------------------------------------------------------------------------|------------|-------|-------|-------------|
| 1     | [leetgo](https://github.com/j178/leetgo)                                           | Go         | 381   | 19    | 2023-05-14  |
| 2     | [chatgpt](https://github.com/j178/chatgpt)                                         | Go         | 354   | 28    | 2023-04-29  |
| 3     | [fanfou-cli](https://github.com/j178/fanfou-cli)                                   | Python     | 22    | 5     | 2017-06-09  |
| 4     | [xiaoai-shutdown-my-computer](https://github.com/j178/xiaoai-shutdown-my-computer) | Python     | 17    | 0     | 2023-04-07  |
| 5     | [tiktoken-go](https://github.com/j178/tiktoken-go)                                 | Go         | 15    | 1     | 2023-04-21  |
| 6     | [ipgw](https://github.com/j178/ipgw)                                               | Python     | 9     | 0     | 2018-09-10  |
| 7     | [github-stargazer](https://github.com/j178/github-stargazer)                       | Go         | 8     | 0     | 2023-03-29  |
| 8     | [2022](https://github.com/j178/2022)                                               | Python     | 6     | 0     | 2022-12-31  |
| 9     | [spotlight](https://github.com/j178/spotlight)                                     | Python     | 4     | 0     | 2019-12-04  |
| 10    | [benchdiff](https://github.com/j178/benchdiff)                                     | Go         | 2     | 0     | 2022-10-30  |
| 11    | [j178](https://github.com/j178/j178)                                               | N/A        | 2     | 1     | 2023-05-14  |
| 12    | [twiyou](https://github.com/j178/twiyou)                                           | Go         | 2     | 0     | 2023-01-09  |
| 13    | [van](https://github.com/j178/van)                                                 | Python     | 2     | 1     | 2020-08-04  |
| 14    | [auto-snatch-course](https://github.com/j178/auto-snatch-course)                   | Python     | 1     | 0     | 2018-04-14  |
| 15    | [course-schedule-icalendar](https://github.com/j178/course-schedule-icalendar)     | Python     | 1     | 0     | 2017-02-10  |
| 16    | [fanfou-bots](https://github.com/j178/fanfou-bots)                                 | Python     | 1     | 1     | 2020-07-23  |
| 17    | [json-tutorial](https://github.com/j178/json-tutorial)                             | C          | 1     | 0     | 2017-01-19  |
| 18    | [leetcode](https://github.com/j178/leetcode)                                       | Go         | 1     | 0     | 2023-05-14  |
| 19    | [naive-sshd](https://github.com/j178/naive-sshd)                                   | Go         | 1     | 0     | 2019-10-15  |
| 20    | [neu6v-crawler](https://github.com/j178/neu6v-crawler)                             | Python     | 1     | 0     | 2016-10-29  |
| 21    | [scoop-bucket](https://github.com/j178/scoop-bucket)                               | PowerShell | 1     | 0     | 2023-05-14  |
| 22    | [v2ex](https://github.com/j178/v2ex)                                               | Python     | 1     | 0     | 2022-07-28  |
| 23    | [GreedySnake](https://github.com/j178/GreedySnake)                                 | Java       | 0     | 1     | 2016-11-01  |
| 24    | [MyWechat](https://github.com/j178/MyWechat)                                       | PHP        | 0     | 1     | 2015-12-24  |
| 25    | [pyrandom](https://github.com/j178/pyrandom)                                       | Python     | 0     | 1     | 2019-04-09  |
| Total |                                                                                    |            | 833   | 59    |             |
<!-- END:created_repos -->

## Repos I've Contributed To

<!-- BEGIN:contributed -->
| No.   | Name                                                                        | First PR                                                           | Last PR                                                            | PR Count                                                                         |
|-------|-----------------------------------------------------------------------------|--------------------------------------------------------------------|--------------------------------------------------------------------|----------------------------------------------------------------------------------|
| 1     | [encode/httpx](https://github.com/encode/httpx)                             | [2019-12-31](https://github.com/encode/httpx/pull/704)             | [2022-07-13](https://github.com/encode/httpx/pull/2302)            | [15](https://github.com/encode/httpx/pulls?q=is%3Apr+author%3Aj178)              |
| 2     | [centrifugal/centrifuge](https://github.com/centrifugal/centrifuge)         | [2022-06-24](https://github.com/centrifugal/centrifuge/pull/230)   | [2022-09-09](https://github.com/centrifugal/centrifuge/pull/252)   | [10](https://github.com/centrifugal/centrifuge/pulls?q=is%3Apr+author%3Aj178)    |
| 3     | [encode/httpcore](https://github.com/encode/httpcore)                       | [2020-08-12](https://github.com/encode/httpcore/pull/154)          | [2022-07-13](https://github.com/encode/httpcore/pull/565)          | [8](https://github.com/encode/httpcore/pulls?q=is%3Apr+author%3Aj178)            |
| 4     | [disksing/twiyou](https://github.com/disksing/twiyou)                       | [2022-10-07](https://github.com/disksing/twiyou/pull/1)            | [2022-12-28](https://github.com/disksing/twiyou/pull/10)           | [7](https://github.com/disksing/twiyou/pulls?q=is%3Apr+author%3Aj178)            |
| 5     | [redis/go-redis](https://github.com/redis/go-redis)                         | [2022-08-01](https://github.com/redis/go-redis/pull/2174)          | [2022-10-05](https://github.com/redis/go-redis/pull/2231)          | [4](https://github.com/redis/go-redis/pulls?q=is%3Apr+author%3Aj178)             |
| 6     | [centrifugal/centrifugo](https://github.com/centrifugal/centrifugo)         | [2022-07-18](https://github.com/centrifugal/centrifugo/pull/525)   | [2022-07-20](https://github.com/centrifugal/centrifugo/pull/528)   | [4](https://github.com/centrifugal/centrifugo/pulls?q=is%3Apr+author%3Aj178)     |
| 7     | [goreleaser/goreleaser](https://github.com/goreleaser/goreleaser)           | [2021-09-03](https://github.com/goreleaser/goreleaser/pull/2455)   | [2023-01-30](https://github.com/goreleaser/goreleaser/pull/3730)   | [3](https://github.com/goreleaser/goreleaser/pulls?q=is%3Apr+author%3Aj178)      |
| 8     | [zurawiki/tiktoken-rs](https://github.com/zurawiki/tiktoken-rs)             | [2023-04-03](https://github.com/zurawiki/tiktoken-rs/pull/14)      | [2023-04-04](https://github.com/zurawiki/tiktoken-rs/pull/15)      | [2](https://github.com/zurawiki/tiktoken-rs/pulls?q=is%3Apr+author%3Aj178)       |
| 9     | [python/cpython](https://github.com/python/cpython)                         | [2021-06-16](https://github.com/python/cpython/pull/26754)         | [2021-12-22](https://github.com/python/cpython/pull/30227)         | [2](https://github.com/python/cpython/pulls?q=is%3Apr+author%3Aj178)             |
| 10    | [rq/rq](https://github.com/rq/rq)                                           | [2019-06-16](https://github.com/rq/rq/pull/1108)                   | [2019-06-16](https://github.com/rq/rq/pull/1109)                   | [2](https://github.com/rq/rq/pulls?q=is%3Apr+author%3Aj178)                      |
| 11    | [caarlos0/fork-cleaner](https://github.com/caarlos0/fork-cleaner)           | [2023-05-11](https://github.com/caarlos0/fork-cleaner/pull/142)    | [2023-05-11](https://github.com/caarlos0/fork-cleaner/pull/142)    | [1](https://github.com/caarlos0/fork-cleaner/pulls?q=is%3Apr+author%3Aj178)      |
| 12    | [mitsuhiko/rye](https://github.com/mitsuhiko/rye)                           | [2023-05-10](https://github.com/mitsuhiko/rye/pull/127)            | [2023-05-10](https://github.com/mitsuhiko/rye/pull/127)            | [1](https://github.com/mitsuhiko/rye/pulls?q=is%3Apr+author%3Aj178)              |
| 13    | [pkoukk/tiktoken-go](https://github.com/pkoukk/tiktoken-go)                 | [2023-04-08](https://github.com/pkoukk/tiktoken-go/pull/5)         | [2023-04-08](https://github.com/pkoukk/tiktoken-go/pull/5)         | [1](https://github.com/pkoukk/tiktoken-go/pulls?q=is%3Apr+author%3Aj178)         |
| 14    | [zurawiki/gptcommit](https://github.com/zurawiki/gptcommit)                 | [2023-04-06](https://github.com/zurawiki/gptcommit/pull/139)       | [2023-04-06](https://github.com/zurawiki/gptcommit/pull/139)       | [1](https://github.com/zurawiki/gptcommit/pulls?q=is%3Apr+author%3Aj178)         |
| 15    | [charmbracelet/bubbletea](https://github.com/charmbracelet/bubbletea)       | [2023-04-03](https://github.com/charmbracelet/bubbletea/pull/709)  | [2023-04-03](https://github.com/charmbracelet/bubbletea/pull/709)  | [1](https://github.com/charmbracelet/bubbletea/pulls?q=is%3Apr+author%3Aj178)    |
| 16    | [sashabaranov/go-openai](https://github.com/sashabaranov/go-openai)         | [2023-03-20](https://github.com/sashabaranov/go-openai/pull/180)   | [2023-03-20](https://github.com/sashabaranov/go-openai/pull/180)   | [1](https://github.com/sashabaranov/go-openai/pulls?q=is%3Apr+author%3Aj178)     |
| 17    | [zellyn/kooky](https://github.com/zellyn/kooky)                             | [2023-01-05](https://github.com/zellyn/kooky/pull/56)              | [2023-01-05](https://github.com/zellyn/kooky/pull/56)              | [1](https://github.com/zellyn/kooky/pulls?q=is%3Apr+author%3Aj178)               |
| 18    | [github/docs](https://github.com/github/docs)                               | [2022-11-10](https://github.com/github/docs/pull/21929)            | [2022-11-10](https://github.com/github/docs/pull/21929)            | [1](https://github.com/github/docs/pulls?q=is%3Apr+author%3Aj178)                |
| 19    | [yihong0618/running_page](https://github.com/yihong0618/running_page)       | [2022-10-13](https://github.com/yihong0618/running_page/pull/319)  | [2022-10-13](https://github.com/yihong0618/running_page/pull/319)  | [1](https://github.com/yihong0618/running_page/pulls?q=is%3Apr+author%3Aj178)    |
| 20    | [pdm-project/pdm](https://github.com/pdm-project/pdm)                       | [2022-10-13](https://github.com/pdm-project/pdm/pull/1434)         | [2022-10-13](https://github.com/pdm-project/pdm/pull/1434)         | [1](https://github.com/pdm-project/pdm/pulls?q=is%3Apr+author%3Aj178)            |
| 21    | [redis/redis-doc](https://github.com/redis/redis-doc)                       | [2022-08-03](https://github.com/redis/redis-doc/pull/2064)         | [2022-08-03](https://github.com/redis/redis-doc/pull/2064)         | [1](https://github.com/redis/redis-doc/pulls?q=is%3Apr+author%3Aj178)            |
| 22    | [Textualize/rich](https://github.com/Textualize/rich)                       | [2022-08-01](https://github.com/Textualize/rich/pull/2437)         | [2022-08-01](https://github.com/Textualize/rich/pull/2437)         | [1](https://github.com/Textualize/rich/pulls?q=is%3Apr+author%3Aj178)            |
| 23    | [centrifugal/centrifuge-go](https://github.com/centrifugal/centrifuge-go)   | [2022-06-29](https://github.com/centrifugal/centrifuge-go/pull/64) | [2022-06-29](https://github.com/centrifugal/centrifuge-go/pull/64) | [1](https://github.com/centrifugal/centrifuge-go/pulls?q=is%3Apr+author%3Aj178)  |
| 24    | [xbin-io/xbin](https://github.com/xbin-io/xbin)                             | [2022-04-25](https://github.com/xbin-io/xbin/pull/2)               | [2022-04-25](https://github.com/xbin-io/xbin/pull/2)               | [1](https://github.com/xbin-io/xbin/pulls?q=is%3Apr+author%3Aj178)               |
| 25    | [EndlessCheng/codeforces-go](https://github.com/EndlessCheng/codeforces-go) | [2022-04-22](https://github.com/EndlessCheng/codeforces-go/pull/3) | [2022-04-22](https://github.com/EndlessCheng/codeforces-go/pull/3) | [1](https://github.com/EndlessCheng/codeforces-go/pulls?q=is%3Apr+author%3Aj178) |
| 26    | [golang/go](https://github.com/golang/go)                                   | [2022-04-06](https://github.com/golang/go/pull/52194)              | [2022-04-06](https://github.com/golang/go/pull/52194)              | [1](https://github.com/golang/go/pulls?q=is%3Apr+author%3Aj178)                  |
| 27    | [yihong0618/GitHubPoster](https://github.com/yihong0618/GitHubPoster)       | [2022-02-15](https://github.com/yihong0618/GitHubPoster/pull/56)   | [2022-02-15](https://github.com/yihong0618/GitHubPoster/pull/56)   | [1](https://github.com/yihong0618/GitHubPoster/pulls?q=is%3Apr+author%3Aj178)    |
| 28    | [gofiber/fiber](https://github.com/gofiber/fiber)                           | [2021-08-30](https://github.com/gofiber/fiber/pull/1510)           | [2021-08-30](https://github.com/gofiber/fiber/pull/1510)           | [1](https://github.com/gofiber/fiber/pulls?q=is%3Apr+author%3Aj178)              |
| 29    | [pydantic/pydantic](https://github.com/pydantic/pydantic)                   | [2021-03-25](https://github.com/pydantic/pydantic/pull/2577)       | [2021-03-25](https://github.com/pydantic/pydantic/pull/2577)       | [1](https://github.com/pydantic/pydantic/pulls?q=is%3Apr+author%3Aj178)          |
| 30    | [urllib3/urllib3](https://github.com/urllib3/urllib3)                       | [2020-11-26](https://github.com/urllib3/urllib3/pull/2095)         | [2020-11-26](https://github.com/urllib3/urllib3/pull/2095)         | [1](https://github.com/urllib3/urllib3/pulls?q=is%3Apr+author%3Aj178)            |
| 31    | [aio-libs/yarl](https://github.com/aio-libs/yarl)                           | [2020-05-20](https://github.com/aio-libs/yarl/pull/452)            | [2020-05-20](https://github.com/aio-libs/yarl/pull/452)            | [1](https://github.com/aio-libs/yarl/pulls?q=is%3Apr+author%3Aj178)              |
| 32    | [7sDream/kd100](https://github.com/7sDream/kd100)                           | [2016-11-16](https://github.com/7sDream/kd100/pull/1)              | [2016-11-16](https://github.com/7sDream/kd100/pull/1)              | [1](https://github.com/7sDream/kd100/pulls?q=is%3Apr+author%3Aj178)              |
| Total |                                                                             |                                                                    |                                                                    | 79                                                                               |
<!-- END:contributed -->
