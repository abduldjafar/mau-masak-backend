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
{% endswagger %}
