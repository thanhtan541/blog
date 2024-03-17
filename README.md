# blog

## Prerequisites
- rbenv
- ruby >= 3.0.3 [Link](https://dev.to/konyu/installing-the-latest-version-of-ruby-on-raspberry-pi-3ofk)
- jekyll + bundle [Link](https://jekyllrb.com/docs/)
- nginx
- linux

## Setup
- systemd service + nginx [Link](https://distroid.net/set-up-jekyll-with-nginx-on-debian/)

## Commands
```bash
# Enable service
$ systemctl enable myblog
$ systemctl start myblog

# Local development
$ bundle exec jekyll serve --livereload --port 5000
```
