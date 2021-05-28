<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>33 Update Employee Education</name>
   <tag></tag>
   <elementGuidId>3e91f917-541d-4304-9e81-76b44f04de4d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;id&quot;,
      &quot;value&quot;: &quot;1&quot;
    },
    {
      &quot;name&quot;: &quot;level&quot;,
      &quot;value&quot;: &quot;2&quot;
    },
    {
      &quot;name&quot;: &quot;seqId&quot;,
      &quot;value&quot;: &quot;2&quot;
    },
    {
      &quot;name&quot;: &quot;institute&quot;,
      &quot;value&quot;: &quot;Universitas Gunadarma&quot;
    },
    {
      &quot;name&quot;: &quot;startDate&quot;,
      &quot;value&quot;: &quot;2009-09-09&quot;
    },
    {
      &quot;name&quot;: &quot;endDateDate&quot;,
      &quot;value&quot;: &quot;2013-09-09&quot;
    },
    {
      &quot;name&quot;: &quot;specialization&quot;,
      &quot;value&quot;: &quot;Economy&quot;
    },
    {
      &quot;name&quot;: &quot;year&quot;,
      &quot;value&quot;: &quot;4&quot;
    },
    {
      &quot;name&quot;: &quot;gpa&quot;,
      &quot;value&quot;: &quot;3.00&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer dbd5faecc93c16921eb5f715ee0b5832977ee960</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${base_url}/api/v1/employee/:id/education</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.base_url</defaultValue>
      <description></description>
      <id>6309dbfe-8737-450b-bdc0-dca28b2fba0b</id>
      <masked>false</masked>
      <name>base_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
