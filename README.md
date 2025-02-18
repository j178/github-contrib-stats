# GitHub Contribution Stats

Generates a contribution summary for your GitHub profile. A Rust implementation of [yihong0618/github-readme-stats](https://github.com/yihong0618/github-readme-stats).

## Visit online

This service is deployed to Vercel, you can visit it in the browser to see your contribution stats: [https://github-contrib-stats.vercel.app](https://github-contrib-stats.vercel.app)

## Use it in GitHub Action

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
        run: ./github-contrib-stats -u ${{ github.repository_owner }} --update README.md
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

## Example

![Repos I created](https://github-contrib-stats.vercel.app/j178/created.svg)
![Repos I contributed to](https://github-contrib-stats.vercel.app/j178/contributed.svg)
