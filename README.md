# GitHub Contribution Stats

Generates a contribution summary for your GitHub profile. A Rust implementation of [yihong0618/github-readme-stats](https://github.com/yihong0618/github-readme-stats).

## Visit online

This service is deployed to Vercel, you can visit it in the browser to see your contribution stats:

- Repos you created: `https://github-contrib-stats.vercel.app/created?username=<username>`
- Repos you contributed: `https://github-contrib-stats.vercel.app/contributed?username=<username>`

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
| 1     | [chatgpt](https://github.com/j178/chatgpt)                                         | Go         | 753   | 52    | 2025-02-11  |
| 2     | [leetgo](https://github.com/j178/leetgo)                                           | Go         | 544   | 34    | 2025-02-11  |
| 3     | [prefligit](https://github.com/j178/prefligit)                                     | Rust       | 80    | 6     | 2025-02-11  |
| 4     | [github-s3](https://github.com/j178/github-s3)                                     | Go         | 52    | 2     | 2025-02-10  |
| 5     | [fanfou-cli](https://github.com/j178/fanfou-cli)                                   | Python     | 23    | 5     | 2017-06-09  |
| 6     | [xiaoai-shutdown-my-computer](https://github.com/j178/xiaoai-shutdown-my-computer) | Python     | 17    | 0     | 2023-04-07  |
| 7     | [tiktoken-go-binding](https://github.com/j178/tiktoken-go-binding)                 | Go         | 15    | 1     | 2023-04-21  |
| 8     | [github-stargazer](https://github.com/j178/github-stargazer)                       | Go         | 11    | 0     | 2024-05-22  |
| 9     | [spotlight](https://github.com/j178/spotlight)                                     | Python     | 5     | 0     | 2023-06-06  |
| 10    | [2022](https://github.com/j178/2022)                                               | Python     | 5     | 0     | 2022-12-31  |
| 11    | [github-contrib-stats](https://github.com/j178/github-contrib-stats)               | Rust       | 4     | 0     | 2025-02-11  |
| 12    | [van](https://github.com/j178/van)                                                 | Python     | 3     | 1     | 2020-08-04  |
| 13    | [auto-snatch-course](https://github.com/j178/auto-snatch-course)                   | Python     | 2     | 0     | 2018-04-14  |
| 14    | [benchdiff](https://github.com/j178/benchdiff)                                     | Go         | 2     | 0     | 2022-10-30  |
| 15    | [git-first](https://github.com/j178/git-first)                                     | Rust       | 2     | 0     | 2023-05-16  |
| 16    | [it](https://github.com/j178/it)                                                   | Go         | 2     | 0     | 2023-12-18  |
| 17    | [opencat-api](https://github.com/j178/opencat-api)                                 | Go         | 2     | 2     | 2024-01-25  |
| 18    | [leetcode](https://github.com/j178/leetcode)                                       | Go         | 1     | 0     | 2025-02-11  |
| 19    | [neu6v-crawler](https://github.com/j178/neu6v-crawler)                             | Python     | 1     | 0     | 2016-10-29  |
| 20    | [json-tutorial](https://github.com/j178/json-tutorial)                             | C          | 1     | 0     | 2017-01-19  |
| 21    | [course-schedule-icalendar](https://github.com/j178/course-schedule-icalendar)     | Python     | 1     | 0     | 2017-02-10  |
| 22    | [fanfou-bots](https://github.com/j178/fanfou-bots)                                 | Python     | 1     | 1     | 2020-07-23  |
| 23    | [naive-sshd](https://github.com/j178/naive-sshd)                                   | Go         | 1     | 0     | 2019-10-15  |
| 24    | [cron-actions](https://github.com/j178/cron-actions)                               | Python     | 1     | 0     | 2023-05-23  |
| 25    | [vvv-scanner](https://github.com/j178/vvv-scanner)                                 | Lua        | 1     | 0     | 2020-08-31  |
| 26    | [j178](https://github.com/j178/j178)                                               | N/A        | 1     | 1     | 2025-02-09  |
| 27    | [v2ex](https://github.com/j178/v2ex)                                               | Python     | 1     | 0     | 2022-07-28  |
| 28    | [scoop-bucket](https://github.com/j178/scoop-bucket)                               | PowerShell | 1     | 0     | 2025-01-22  |
| 29    | [2023](https://github.com/j178/2023)                                               | N/A        | 1     | 0     | 2025-02-11  |
| 30    | [llms](https://github.com/j178/llms)                                               | Go         | 1     | 0     | 2024-03-09  |
| 31    | [MyWechat](https://github.com/j178/MyWechat)                                       | PHP        | 0     | 1     | 2015-12-24  |
| 32    | [pyrandom](https://github.com/j178/pyrandom)                                       | Python     | 0     | 1     | 2019-04-09  |
| 33    | [GreedySnake](https://github.com/j178/GreedySnake)                                 | Java       | 0     | 1     | 2016-11-01  |
| Total |                                                                                    |            | 1535  | 108   |             |
<!-- END:created_repos -->

## Repos I've Contributed To

<!-- BEGIN:contributed -->
| No.   | Name                                                                                | Stars  | First PR                                                                | Last PR                                                                 | PR Count                                                                             |
|-------|-------------------------------------------------------------------------------------|--------|-------------------------------------------------------------------------|-------------------------------------------------------------------------|--------------------------------------------------------------------------------------|
| 1     | [astral-sh/uv](https://github.com/astral-sh/uv)                                     | 38977  | [2024-02-26](https://github.com/astral-sh/uv/pull/1979)                 | [2025-02-05](https://github.com/astral-sh/uv/pull/11240)                | [79](https://github.com/astral-sh/uv/pulls?q=is%3Apr+author%3Aj178)                  |
| 2     | [astral-sh/rye](https://github.com/astral-sh/rye)                                   | 14015  | [2023-05-10](https://github.com/astral-sh/rye/pull/127)                 | [2024-04-02](https://github.com/astral-sh/rye/pull/983)                 | [41](https://github.com/astral-sh/rye/pulls?q=is%3Apr+author%3Aj178)                 |
| 3     | [encode/httpx](https://github.com/encode/httpx)                                     | 13673  | [2019-12-31](https://github.com/encode/httpx/pull/704)                  | [2022-07-13](https://github.com/encode/httpx/pull/2302)                 | [15](https://github.com/encode/httpx/pulls?q=is%3Apr+author%3Aj178)                  |
| 4     | [centrifugal/centrifuge](https://github.com/centrifugal/centrifuge)                 | 1140   | [2022-06-24](https://github.com/centrifugal/centrifuge/pull/230)        | [2022-09-09](https://github.com/centrifugal/centrifuge/pull/252)        | [10](https://github.com/centrifugal/centrifuge/pulls?q=is%3Apr+author%3Aj178)        |
| 5     | [goreleaser/goreleaser](https://github.com/goreleaser/goreleaser)                   | 14224  | [2021-09-03](https://github.com/goreleaser/goreleaser/pull/2455)        | [2024-04-04](https://github.com/goreleaser/goreleaser/pull/4750)        | [8](https://github.com/goreleaser/goreleaser/pulls?q=is%3Apr+author%3Aj178)          |
| 6     | [encode/httpcore](https://github.com/encode/httpcore)                               | 480    | [2020-08-12](https://github.com/encode/httpcore/pull/154)               | [2022-07-13](https://github.com/encode/httpcore/pull/565)               | [8](https://github.com/encode/httpcore/pulls?q=is%3Apr+author%3Aj178)                |
| 7     | [disksing/twiyou](https://github.com/disksing/twiyou)                               | 131    | [2022-10-07](https://github.com/disksing/twiyou/pull/1)                 | [2022-12-28](https://github.com/disksing/twiyou/pull/10)                | [7](https://github.com/disksing/twiyou/pulls?q=is%3Apr+author%3Aj178)                |
| 8     | [centrifugal/centrifugo](https://github.com/centrifugal/centrifugo)                 | 8719   | [2022-07-18](https://github.com/centrifugal/centrifugo/pull/525)        | [2024-01-12](https://github.com/centrifugal/centrifugo/pull/762)        | [5](https://github.com/centrifugal/centrifugo/pulls?q=is%3Apr+author%3Aj178)         |
| 9     | [redis/go-redis](https://github.com/redis/go-redis)                                 | 20461  | [2022-08-01](https://github.com/redis/go-redis/pull/2174)               | [2022-10-05](https://github.com/redis/go-redis/pull/2231)               | [4](https://github.com/redis/go-redis/pulls?q=is%3Apr+author%3Aj178)                 |
| 10    | [sashabaranov/go-openai](https://github.com/sashabaranov/go-openai)                 | 9560   | [2023-03-20](https://github.com/sashabaranov/go-openai/pull/180)        | [2023-06-15](https://github.com/sashabaranov/go-openai/pull/374)        | [3](https://github.com/sashabaranov/go-openai/pulls?q=is%3Apr+author%3Aj178)         |
| 11    | [redis/rueidis](https://github.com/redis/rueidis)                                   | 2545   | [2023-12-15](https://github.com/redis/rueidis/pull/426)                 | [2024-06-12](https://github.com/redis/rueidis/pull/561)                 | [2](https://github.com/redis/rueidis/pulls?q=is%3Apr+author%3Aj178)                  |
| 12    | [pdm-project/pdm](https://github.com/pdm-project/pdm)                               | 8107   | [2022-10-13](https://github.com/pdm-project/pdm/pull/1434)              | [2024-04-03](https://github.com/pdm-project/pdm/pull/2766)              | [2](https://github.com/pdm-project/pdm/pulls?q=is%3Apr+author%3Aj178)                |
| 13    | [1Password/shell-plugins](https://github.com/1Password/shell-plugins)               | 550    | [2023-05-29](https://github.com/1Password/shell-plugins/pull/271)       | [2023-05-29](https://github.com/1Password/shell-plugins/pull/273)       | [2](https://github.com/1Password/shell-plugins/pulls?q=is%3Apr+author%3Aj178)        |
| 14    | [apache/opendal](https://github.com/apache/opendal)                                 | 3766   | [2023-05-24](https://github.com/apache/opendal/pull/2307)               | [2023-05-24](https://github.com/apache/opendal/pull/2308)               | [2](https://github.com/apache/opendal/pulls?q=is%3Apr+author%3Aj178)                 |
| 15    | [zurawiki/tiktoken-rs](https://github.com/zurawiki/tiktoken-rs)                     | 289    | [2023-04-03](https://github.com/zurawiki/tiktoken-rs/pull/14)           | [2023-04-04](https://github.com/zurawiki/tiktoken-rs/pull/15)           | [2](https://github.com/zurawiki/tiktoken-rs/pulls?q=is%3Apr+author%3Aj178)           |
| 16    | [yihong0618/GitHubPoster](https://github.com/yihong0618/GitHubPoster)               | 1798   | [2022-02-14](https://github.com/yihong0618/GitHubPoster/pull/55)        | [2022-02-15](https://github.com/yihong0618/GitHubPoster/pull/56)        | [2](https://github.com/yihong0618/GitHubPoster/pulls?q=is%3Apr+author%3Aj178)        |
| 17    | [python/cpython](https://github.com/python/cpython)                                 | 65195  | [2021-06-16](https://github.com/python/cpython/pull/26754)              | [2021-12-22](https://github.com/python/cpython/pull/30227)              | [2](https://github.com/python/cpython/pulls?q=is%3Apr+author%3Aj178)                 |
| 18    | [rq/rq](https://github.com/rq/rq)                                                   | 10015  | [2019-06-16](https://github.com/rq/rq/pull/1108)                        | [2019-06-16](https://github.com/rq/rq/pull/1109)                        | [2](https://github.com/rq/rq/pulls?q=is%3Apr+author%3Aj178)                          |
| 19    | [bradfitz/go-tool-cache](https://github.com/bradfitz/go-tool-cache)                 | 96     | [2024-12-16](https://github.com/bradfitz/go-tool-cache/pull/10)         | [2024-12-16](https://github.com/bradfitz/go-tool-cache/pull/10)         | [1](https://github.com/bradfitz/go-tool-cache/pulls?q=is%3Apr+author%3Aj178)         |
| 20    | [or-shachar/go-tool-cache](https://github.com/or-shachar/go-tool-cache)             | 12     | [2024-12-16](https://github.com/or-shachar/go-tool-cache/pull/15)       | [2024-12-16](https://github.com/or-shachar/go-tool-cache/pull/15)       | [1](https://github.com/or-shachar/go-tool-cache/pulls?q=is%3Apr+author%3Aj178)       |
| 21    | [redis/redis](https://github.com/redis/redis)                                       | 67953  | [2024-06-12](https://github.com/redis/redis/pull/13339)                 | [2024-06-12](https://github.com/redis/redis/pull/13339)                 | [1](https://github.com/redis/redis/pulls?q=is%3Apr+author%3Aj178)                    |
| 22    | [nalgeon/redka](https://github.com/nalgeon/redka)                                   | 3599   | [2024-06-07](https://github.com/nalgeon/redka/pull/26)                  | [2024-06-07](https://github.com/nalgeon/redka/pull/26)                  | [1](https://github.com/nalgeon/redka/pulls?q=is%3Apr+author%3Aj178)                  |
| 23    | [tebeka/expmod](https://github.com/tebeka/expmod)                                   | 27     | [2023-11-18](https://github.com/tebeka/expmod/pull/1)                   | [2023-11-18](https://github.com/tebeka/expmod/pull/1)                   | [1](https://github.com/tebeka/expmod/pulls?q=is%3Apr+author%3Aj178)                  |
| 24    | [smallnest/smallchat](https://github.com/smallnest/smallchat)                       | 91     | [2023-10-29](https://github.com/smallnest/smallchat/pull/1)             | [2023-10-29](https://github.com/smallnest/smallchat/pull/1)             | [1](https://github.com/smallnest/smallchat/pulls?q=is%3Apr+author%3Aj178)            |
| 25    | [fish-shell/fish-shell](https://github.com/fish-shell/fish-shell)                   | 27937  | [2023-06-02](https://github.com/fish-shell/fish-shell/pull/9825)        | [2023-06-02](https://github.com/fish-shell/fish-shell/pull/9825)        | [1](https://github.com/fish-shell/fish-shell/pulls?q=is%3Apr+author%3Aj178)          |
| 26    | [yihong0618/github-readme-stats](https://github.com/yihong0618/github-readme-stats) | 141    | [2023-05-14](https://github.com/yihong0618/github-readme-stats/pull/13) | [2023-05-14](https://github.com/yihong0618/github-readme-stats/pull/13) | [1](https://github.com/yihong0618/github-readme-stats/pulls?q=is%3Apr+author%3Aj178) |
| 27    | [caarlos0/fork-cleaner](https://github.com/caarlos0/fork-cleaner)                   | 332    | [2023-05-11](https://github.com/caarlos0/fork-cleaner/pull/142)         | [2023-05-11](https://github.com/caarlos0/fork-cleaner/pull/142)         | [1](https://github.com/caarlos0/fork-cleaner/pulls?q=is%3Apr+author%3Aj178)          |
| 28    | [pkoukk/tiktoken-go](https://github.com/pkoukk/tiktoken-go)                         | 709    | [2023-04-08](https://github.com/pkoukk/tiktoken-go/pull/5)              | [2023-04-08](https://github.com/pkoukk/tiktoken-go/pull/5)              | [1](https://github.com/pkoukk/tiktoken-go/pulls?q=is%3Apr+author%3Aj178)             |
| 29    | [zurawiki/gptcommit](https://github.com/zurawiki/gptcommit)                         | 2355   | [2023-04-06](https://github.com/zurawiki/gptcommit/pull/139)            | [2023-04-06](https://github.com/zurawiki/gptcommit/pull/139)            | [1](https://github.com/zurawiki/gptcommit/pulls?q=is%3Apr+author%3Aj178)             |
| 30    | [charmbracelet/bubbletea](https://github.com/charmbracelet/bubbletea)               | 29472  | [2023-04-03](https://github.com/charmbracelet/bubbletea/pull/709)       | [2023-04-03](https://github.com/charmbracelet/bubbletea/pull/709)       | [1](https://github.com/charmbracelet/bubbletea/pulls?q=is%3Apr+author%3Aj178)        |
| 31    | [browserutils/kooky](https://github.com/browserutils/kooky)                         | 226    | [2023-01-05](https://github.com/browserutils/kooky/pull/56)             | [2023-01-05](https://github.com/browserutils/kooky/pull/56)             | [1](https://github.com/browserutils/kooky/pulls?q=is%3Apr+author%3Aj178)             |
| 32    | [github/docs](https://github.com/github/docs)                                       | 16945  | [2022-11-10](https://github.com/github/docs/pull/21929)                 | [2022-11-10](https://github.com/github/docs/pull/21929)                 | [1](https://github.com/github/docs/pulls?q=is%3Apr+author%3Aj178)                    |
| 33    | [yihong0618/running_page](https://github.com/yihong0618/running_page)               | 3749   | [2022-10-13](https://github.com/yihong0618/running_page/pull/319)       | [2022-10-13](https://github.com/yihong0618/running_page/pull/319)       | [1](https://github.com/yihong0618/running_page/pulls?q=is%3Apr+author%3Aj178)        |
| 34    | [redis/redis-doc](https://github.com/redis/redis-doc)                               | 2309   | [2022-08-03](https://github.com/redis/redis-doc/pull/2064)              | [2022-08-03](https://github.com/redis/redis-doc/pull/2064)              | [1](https://github.com/redis/redis-doc/pulls?q=is%3Apr+author%3Aj178)                |
| 35    | [Textualize/rich](https://github.com/Textualize/rich)                               | 50650  | [2022-08-01](https://github.com/Textualize/rich/pull/2437)              | [2022-08-01](https://github.com/Textualize/rich/pull/2437)              | [1](https://github.com/Textualize/rich/pulls?q=is%3Apr+author%3Aj178)                |
| 36    | [centrifugal/centrifuge-go](https://github.com/centrifugal/centrifuge-go)           | 217    | [2022-06-29](https://github.com/centrifugal/centrifuge-go/pull/64)      | [2022-06-29](https://github.com/centrifugal/centrifuge-go/pull/64)      | [1](https://github.com/centrifugal/centrifuge-go/pulls?q=is%3Apr+author%3Aj178)      |
| 37    | [xbin-io/xbin](https://github.com/xbin-io/xbin)                                     | 247    | [2022-04-25](https://github.com/xbin-io/xbin/pull/2)                    | [2022-04-25](https://github.com/xbin-io/xbin/pull/2)                    | [1](https://github.com/xbin-io/xbin/pulls?q=is%3Apr+author%3Aj178)                   |
| 38    | [EndlessCheng/codeforces-go](https://github.com/EndlessCheng/codeforces-go)         | 5720   | [2022-04-22](https://github.com/EndlessCheng/codeforces-go/pull/3)      | [2022-04-22](https://github.com/EndlessCheng/codeforces-go/pull/3)      | [1](https://github.com/EndlessCheng/codeforces-go/pulls?q=is%3Apr+author%3Aj178)     |
| 39    | [golang/go](https://github.com/golang/go)                                           | 125638 | [2022-04-06](https://github.com/golang/go/pull/52194)                   | [2022-04-06](https://github.com/golang/go/pull/52194)                   | [1](https://github.com/golang/go/pulls?q=is%3Apr+author%3Aj178)                      |
| 40    | [gofiber/fiber](https://github.com/gofiber/fiber)                                   | 34958  | [2021-08-30](https://github.com/gofiber/fiber/pull/1510)                | [2021-08-30](https://github.com/gofiber/fiber/pull/1510)                | [1](https://github.com/gofiber/fiber/pulls?q=is%3Apr+author%3Aj178)                  |
| 41    | [pydantic/pydantic](https://github.com/pydantic/pydantic)                           | 22387  | [2021-03-25](https://github.com/pydantic/pydantic/pull/2577)            | [2021-03-25](https://github.com/pydantic/pydantic/pull/2577)            | [1](https://github.com/pydantic/pydantic/pulls?q=is%3Apr+author%3Aj178)              |
| 42    | [urllib3/urllib3](https://github.com/urllib3/urllib3)                               | 3836   | [2020-11-26](https://github.com/urllib3/urllib3/pull/2095)              | [2020-11-26](https://github.com/urllib3/urllib3/pull/2095)              | [1](https://github.com/urllib3/urllib3/pulls?q=is%3Apr+author%3Aj178)                |
| 43    | [aio-libs/yarl](https://github.com/aio-libs/yarl)                                   | 1366   | [2020-05-20](https://github.com/aio-libs/yarl/pull/452)                 | [2020-05-20](https://github.com/aio-libs/yarl/pull/452)                 | [1](https://github.com/aio-libs/yarl/pulls?q=is%3Apr+author%3Aj178)                  |
| 44    | [7sDream/kd100](https://github.com/7sDream/kd100)                                   | 28     | [2016-11-16](https://github.com/7sDream/kd100/pull/1)                   | [2016-11-16](https://github.com/7sDream/kd100/pull/1)                   | [1](https://github.com/7sDream/kd100/pulls?q=is%3Apr+author%3Aj178)                  |
| Total |                                                                                     |        |                                                                         |                                                                         | 222                                                                                  |
<!-- END:contributed -->
