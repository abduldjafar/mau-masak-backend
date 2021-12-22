# Delete Content Tags, Sub Tags, Sub Sub Tags

{% swagger method="delete" path="/v1/users/content/category/:kind/:id" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="path" name="kind" required="true" %}
can chosee tags,sub_

_tags, or sub_

\_sub_tags
{% endswagger-parameter %}

{% swagger-parameter in="path" name="id" required="true" %}
tags id, sub 

_tags id, or sub sub_

 tags depend on 'kind' value
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
        "code": 400,
        "data": null,
        "isError": true,
        "message": "from server"
    }
}
```
{% endswagger-response %}
{% endswagger %}
