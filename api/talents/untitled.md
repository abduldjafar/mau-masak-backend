# Get Talents By ID

{% api-method method="get" host="http://" path="backend-services-prdd5cupna-as.a.run.app/v1/talents/:id" %}
{% api-method-summary %}
get talents by id
{% endapi-method-summary %}

{% api-method-description %}
This methods allow users for seeing talents datas
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-path-parameters %}
{% api-method-parameter name="id" type="string" required=true %}
user id
{% endapi-method-parameter %}
{% endapi-method-path-parameters %}

{% api-method-headers %}
{% api-method-parameter name="Authorization" type="string" required=true %}
for auth. fill use token that have from login
{% endapi-method-parameter %}
{% endapi-method-headers %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}

{% endapi-method-response-example-description %}

```

```
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}

