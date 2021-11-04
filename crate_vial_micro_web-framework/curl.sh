curl 127.0.0.1:2333
# Hello, world!
curl 127.0.0.1:2333/echo -d 'echo=sean'
# <h1>sean</h1>
curl -X POST 127.0.0.1:2333/echo -d "echo=sean"
# <h1>sean</h1>
