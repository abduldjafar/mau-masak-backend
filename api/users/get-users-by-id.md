# Get Users By ID

{% api-method method="get" host="https://api-dev.mico.sg/v1/users/:id" path="" %}
{% api-method-summary %}
get users by id
{% endapi-method-summary %}

{% api-method-description %}

{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-path-parameters %}
{% api-method-parameter name="id" type="string" required=false %}
user id
{% endapi-method-parameter %}
{% endapi-method-path-parameters %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}

{% endapi-method-response-example-description %}

```
{
    "responses": {
        "code":200,
        "isError":false,
        "message":"success",
        "data":{
            "email": "koteka@gmail.com",
            "name": "koteka exchain",
        }
    }
}
```
{% endapi-method-response-example %}

{% api-method-response-example httpCode=404 %}
{% api-method-response-example-description %}

{% endapi-method-response-example-description %}

```
{
    "responses": {
        "code":404,
        "isError":true,
        "message":"user not found",
        "data":nil
    }
}
```
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}

