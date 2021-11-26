# upload image item

{% swagger method="post" path="/v1/storage" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="body" name="doc_type" type="string" required="true" %}
form-data (value : "item")
{% endswagger-parameter %}

{% swagger-parameter in="body" name="filename" type="file" required="true" %}
form-data(value:"file path")
{% endswagger-parameter %}

{% swagger-parameter in="body" name="itemid" type="string" required="true" %}
form-data(value:"item id")
{% endswagger-parameter %}
{% endswagger %}

### Example

{% tabs %}
{% tab title="curl" %}
```
curl --location --request POST 'http://localhost:8080/v1/storage' \
--header 'Authorization: xxxxxx' \
--header 'Cookie: session=eyJfZnJlc2giOmZhbHNlLCJfcGVybWFuZW50Ijp0cnVlfQ.YZ9iLg.RIReFLpRPKEtZXjdKFqfvtmhJPg' \
--form 'doc_type="item"' \
--form 'filename=@"/Users/kotekaman/Documents/Pictures/1200px-Prometheus_software_logo.svg.png"' \
--form 'itemid="61a0632a07eb249066613e81"'
```
{% endtab %}

{% tab title="javascript-fetch" %}
```javascript
var myHeaders = new Headers();
myHeaders.append("Authorization", "xxxxxx");
myHeaders.append("Cookie", "session=eyJfZnJlc2giOmZhbHNlLCJfcGVybWFuZW50Ijp0cnVlfQ.YZ9iLg.RIReFLpRPKEtZXjdKFqfvtmhJPg");

var formdata = new FormData();
formdata.append("doc_type", "item");
formdata.append("filename", fileInput.files[0], "1200px-Prometheus_software_logo.svg.png");
formdata.append("itemid", "61a0632a07eb249066613e81");

var requestOptions = {
  method: 'POST',
  headers: myHeaders,
  body: formdata,
  redirect: 'follow'
};

fetch("http://localhost:8080/v1/storage", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
{% endtab %}
{% endtabs %}
