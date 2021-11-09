# upload thumbnail

{% swagger method="post" path="/v1/storage" baseUrl="" summary="This Post use  form-data format" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="body" name="filename" type="file" required="true" %}

{% endswagger-parameter %}

{% swagger-parameter in="body" name="doc_type" type="string" %}
kind of file (use 'thumbnail')
{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": "https://storage.googleapis.com/mico_project/thumbnail/image_thumbnail/lunchbox.jpeg",
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

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request POST 'http://localhost:8080/v1/storage'
--form 'filename=@"/Users/kotekaman/Downloads/drive-download-20211103T130723Z-001/lunchbox.jpeg"'
--form 'doc_type="thumbnail"'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
