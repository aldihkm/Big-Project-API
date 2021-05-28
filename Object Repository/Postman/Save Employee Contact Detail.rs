<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Save Employee Contact Detail</name>
   <tag></tag>
   <elementGuidId>76a27faa-afe5-4d8f-8518-040534a76fb7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;id&quot;,
      &quot;value&quot;: &quot;${id}&quot;
    },
    {
      &quot;name&quot;: &quot;addressStreet1&quot;,
      &quot;value&quot;: &quot;Pondok Melati&quot;
    },
    {
      &quot;name&quot;: &quot;addressStreet2&quot;,
      &quot;value&quot;: &quot;Harapan Baru&quot;
    },
    {
      &quot;name&quot;: &quot;city&quot;,
      &quot;value&quot;: &quot;Bekasi&quot;
    },
    {
      &quot;name&quot;: &quot;state&quot;,
      &quot;value&quot;: &quot;Jawa Barat&quot;
    },
    {
      &quot;name&quot;: &quot;zip&quot;,
      &quot;value&quot;: &quot;17415&quot;
    },
    {
      &quot;name&quot;: &quot;country&quot;,
      &quot;value&quot;: &quot;Indonesia&quot;
    },
    {
      &quot;name&quot;: &quot;homeTelephone&quot;,
      &quot;value&quot;: &quot;0218472628&quot;
    },
    {
      &quot;name&quot;: &quot;mobile&quot;,
      &quot;value&quot;: &quot;085697836699&quot;
    },
    {
      &quot;name&quot;: &quot;workTelephone&quot;,
      &quot;value&quot;: &quot;021254345&quot;
    },
    {
      &quot;name&quot;: &quot;workEmail&quot;,
      &quot;value&quot;: &quot;aldi@mailnesia.com&quot;
    },
    {
      &quot;name&quot;: &quot;otherEmail&quot;,
      &quot;value&quot;: &quot;hakim@mailnesia.com&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/api/v1/employee/${id}/contact-detail</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>6bf102ca-cf7f-4a8c-8699-9af85a77497e</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>ebfd0b34-b1fb-4a2f-af46-12d61cbdaacd</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id</defaultValue>
      <description></description>
      <id>0a5e3ee7-5086-450e-b301-8200535f2069</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
