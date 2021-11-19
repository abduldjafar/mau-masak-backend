# create sub tags

{% swagger method="post" path="/v1/users/content/sub_tags" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}
{% endswagger %}

### Json Body

{% tabs %}
{% tab title="First Tab" %}
```
{
    "conten_tags_id":"61780c138fcc63ed963c8cf8",
    "name":"sub_asoi"
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
curl --location --request POST 'http://localhost:8080/v1/users/content/sub_tags' \
--header 'Authorization: xxxxxx' \
--header 'Content-Type: application/json' \
--data-raw '{
    "conten_tags_id":"61780c138fcc63ed963c8cf8",
    "name":"sub_asoi"
}'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
