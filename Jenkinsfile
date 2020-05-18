pipeline {
  agent { label 'UbuntuVM' }
  parameters {
    gitParameter name: 'TAG', 
                 type: 'PT_TAG',
                 defaultValue: 'master'
  }

  stages {
    stage('Checkout Git TAG') {
      steps {
        checkout([$class: 'GitSCM',
                  branches: [[name: "${params.TAG}"]],
                  doGenerateSubmoduleConfigurations: false,
                  extensions: [],
                  gitTool: 'Default',
                  submoduleCfg: [],
                  userRemoteConfigs: [[url: 'https://github.com/atolab/eclipse-zenoh-java.git']]
                ])
      }
    }
    stage('Release build') {
      steps {
        sh '''
          make all-cross
        '''
      }
    }
  }

  post {
    success {
        archiveArtifacts artifacts: 'zenoh/target/zenoh-*.jar, examples/*/target/zenoh-*.jar', fingerprint: true
    }
  }
}
