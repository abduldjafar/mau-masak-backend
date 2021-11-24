# get persona questions by questions number

{% swagger method="get" path="/v1/admin/persona_questions/:number" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Auhtorizarion" required="true" %}

{% endswagger-parameter %}

{% swagger-parameter in="path" name="number" type="int" required="true" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": {
            "question_number": 1,
            "questin_string": "I go by?",
            "answer": {
                "a": "Mr.",
                "b": "Mrs.",
                "c": "Mrx.",
                "d": "",
                "e": "",
                "f": ""
            }
        },
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}

### Example

{% tabs %}
{% tab title="curl" %}
```
curl --location --request GET 'http://localhost:8080/v1/admin/persona_questions/1' \
--header 'Authorization: ".eyJpZCI6IjYxOGNkYTEwZDIxYTYzNzUwZWU5NzY5OSIsImVtYWlsIjoiYWRtaW5AbWljby5lYXJ0aCIsImRhdGF0eXBlIjoiYWRtaW4iLCJleHAiOjE2Mzc3Mzc4NTR9.oxZ0PQBX6mDcUUTLsRJNWPfdZohH-1eS6dLxwEfKrmE'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
