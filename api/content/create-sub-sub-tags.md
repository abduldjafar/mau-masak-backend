# create sub sub tags

{% swagger method="post" path="/v1/users/content/sub_sub_tags" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": "6197724102304889d352210d",
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}

### Json Body

{% tabs %}
{% tab title="body" %}
```
{
    "conten_tags_id":"",
    "conten_sub_tags_id":"",
    "name":""
}
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```
curl --location --request POST 'http://localhost:8080/v1/users/content/sub_sub_tags' \
--header 'Authorization: xxxxx' \
--header 'Content-Type: application/json' \
--data-raw '{
    "conten_tags_id":"",
    "conten_sub_tags_id":"",
    "name":""
}'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
