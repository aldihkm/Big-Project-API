<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>08 Update Employee Contact Detail</name>
   <tag></tag>
   <elementGuidId>d65d5138-6264-4445-9dd6-45c14cbf2870</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;id&quot;,
      &quot;value&quot;: &quot;25&quot;
    },
    {
      &quot;name&quot;: &quot;addressStreet1&quot;,
      &quot;value&quot;: &quot;Pondok Melati Indah&quot;
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
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer 7b8e3a82966bae8c038db8d843c43ac12e393078</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${base_url}/api/v1/employee/:id/contact-detail</restUrl>
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
      <id>7ad498d1-c612-4ef4-b936-93e04cc7a479</id>
      <masked>false</masked>
      <name>base_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
