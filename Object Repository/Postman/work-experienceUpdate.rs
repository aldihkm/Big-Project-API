<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>work-experienceUpdate</name>
   <tag></tag>
   <elementGuidId>5f1a0086-0336-4df4-9782-141dae59af4c</elementGuidId>
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
      &quot;name&quot;: &quot;seqId&quot;,
      &quot;value&quot;: &quot;1&quot;
    },
    {
      &quot;name&quot;: &quot;company&quot;,
      &quot;value&quot;: &quot;PT Ikon&quot;
    },
    {
      &quot;name&quot;: &quot;title&quot;,
      &quot;value&quot;: &quot;QA&quot;
    },
    {
      &quot;name&quot;: &quot;fromDate&quot;,
      &quot;value&quot;: &quot;2021-09-09&quot;
    },
    {
      &quot;name&quot;: &quot;toDate&quot;,
      &quot;value&quot;: &quot;2099-09-09&quot;
    },
    {
      &quot;name&quot;: &quot;comment&quot;,
      &quot;value&quot;: &quot;kerja&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer 86a6a554bc2f7120d0d8db1f4ee4e0562590f116</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${base_url}/api/v1/employee/:id/work-experience</restUrl>
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
      <id>c05d0449-f42a-41ae-bd11-3ba77702537e</id>
      <masked>false</masked>
      <name>base_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
