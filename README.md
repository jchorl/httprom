# httprom
It just makes HTTP requests and meters the results using prometheus push

## Usage
```bash
docker run -it --rm \
    jchorl/httprom \
    -X POST \
    --metrics-prefix mycron \
    --prometheus-push-addr http://prom:9091 \
    https://webhook.site/94590af7-d737-4f49-9a39-35249f6b1d65
```
