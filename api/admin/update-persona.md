# Update Persona

{% swagger method="put" path="/v1/admin/persona/:persona_id" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Auhorization" required="true" %}

{% endswagger-parameter %}

{% swagger-parameter in="path" name="persona_id" required="true" %}

{% endswagger-parameter %}
{% endswagger %}

### Json Body

{% tabs %}
{% tab title="Example" %}
```
// Some code
{
    "persona_name": "asoi",
    "persona_number": 15,
    "persona_answer_five":"a"
}
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
