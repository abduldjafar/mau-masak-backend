# create

{% swagger method="post" path="/v1/users/content" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": null,
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}

{% swagger-response status="400: Bad Request" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": null,
        "isError": true,
        "message": "from server"
    }
}
```
{% endswagger-response %}
{% endswagger %}

### Body Request

{% tabs %}
{% tab title="Json Body" %}
```javascript
{
    "body_content": {
        "time": 1635340384901,
        "blocks": [
            {
                "id": "cnWnScqSmU",
                "type": "header",
                "data": {
                    "text": "Editor.js",
                    "level": 2
                }
            },
            {
                "id": "JeUnLY4T3c",
                "type": "paragraph",
                "data": {
                    "text": "Hey. Meet the new Editor. On this page you can see it in action — try to edit this text."
                }
            },
            {
                "id": "Oz3wIhv6aB",
                "type": "header",
                "data": {
                    "text": "Key features",
                    "level": 3
                }
            },
            {
                "id": "FrOMoGVygB",
                "type": "list",
                "data": {
                    "style": "unordered",
                    "items": [
                        "It is a block-styled editor",
                        "It returns clean data output in JSON",
                        "Designed to be extendable and pluggable with a simple API"
                    ]
                }
            },
            {
                "id": "XoC_BywMpi",
                "type": "header",
                "data": {
                    "text": "What does it mean «block-styled editor»",
                    "level": 3
                }
            },
            {
                "id": "iSAr6LBOex",
                "type": "paragraph",
                "data": {
                    "text": "Workspace in classic editors is made of a single contenteditable element, used to create different HTML markups. Editor.js <mark class=\"cdx-marker\">workspace consists of separate Blocks: paragraphs, headings, images, lists, quotes, etc</mark>. Each of them is an independent contenteditable element (or more complex structure) provided by Plugin and united by Editor's Core."
                }
            },
            {
                "id": "QdlJztZ0Qn",
                "type": "paragraph",
                "data": {
                    "text": "There are dozens of <a href=\"https://github.com/editor-js\">ready-to-use Blocks</a> and the <a href=\"https://editorjs.io/creating-a-block-tool\">simple API</a> for creation any Block you need. For example, you can implement Blocks for Tweets, Instagram posts, surveys and polls, CTA-buttons and even games."
                }
            },
            {
                "id": "s7hCjQPfWU",
                "type": "header",
                "data": {
                    "text": "What does it mean clean data output",
                    "level": 3
                }
            },
            {
                "id": "xf_e1-TuUm",
                "type": "paragraph",
                "data": {
                    "text": "Classic WYSIWYG-editors produce raw HTML-markup with both content data and content appearance. On the contrary, Editor.js outputs JSON object with data of each Block. You can see an example below"
                }
            },
            {
                "id": "fgPjENqpn-",
                "type": "paragraph",
                "data": {
                    "text": "Given data can be used as you want: render with HTML for <code class=\"inline-code\">Web clients</code>, render natively for <code class=\"inline-code\">mobile apps</code>, create markup for <code class=\"inline-code\">Facebook Instant Articles</code> or <code class=\"inline-code\">Google AMP</code>, generate an <code class=\"inline-code\">audio version</code> and so on."
                }
            },
            {
                "id": "BO2ep2chOL",
                "type": "paragraph",
                "data": {
                    "text": "Clean data is useful to sanitize, validate and process on the backend."
                }
            },
            {
                "id": "vfrMthT9Ja",
                "type": "delimiter",
                "data": {}
            },
            {
                "id": "nXfQV1Vj9X",
                "type": "paragraph",
                "data": {
                    "text": "We have been working on this project more than three years. Several large media projects help us to test and debug the Editor, to make it's core more stable. At the same time we significantly improved the API. Now, it can be used to create any plugin for any task. Hope you enjoy. 😏"
                }
            },
            {
                "id": "Kig9SXN_Fw",
                "type": "image",
                "data": {
                    "file": {
                        "url": "https://codex.so/public/app/img/external/codex2x.png"
                    },
                    "caption": "",
                    "withBorder": false,
                    "stretched": false,
                    "withBackground": false
                }
            }
        ],
        "version": "2.22.2"
    },
    "title": "",
    "country": "",
    "sub_tags": [
        ""
    ],
    "tags": [
        ""
    ]
}
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
