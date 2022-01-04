# download report by vendor

{% swagger method="get" path="/v1/admin/order_reports/vendor/:filename" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="path" name="filename" type="string" required="true" %}

{% endswagger-parameter %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="end_date" required="true" %}
example : 2022-01-01T23:59:59Z
{% endswagger-parameter %}

{% swagger-parameter in="query" name="start_date" required="true" %}
example : 2021-12-01T00:00:00Z
{% endswagger-parameter %}

{% swagger-parameter in="query" name="vendor_id" required="true" %}

{% endswagger-parameter %}
{% endswagger %}
