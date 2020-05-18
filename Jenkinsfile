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
        cleanWs()
        checkout([$class: 'GitSCM',
                  branches: [[name: "${params.TAG}"]],
                  doGenerateSubmoduleConfigurations: false,
                  extensions: [[$class: 'SubmoduleOption',
                    depth: 1,
                    disableSubmodules: false,
                    parentCredentials: false,
                    recursiveSubmodules: true,
                    reference: '',
                    shallow: true,
                    trackingSubmodules: true]],
                  gitTool: 'Default',
                  submoduleCfg: [],
                  userRemoteConfigs: [[url: 'https://github.com/atolab/eclipse-zenoh-python.git']]
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
        archiveArtifacts artifacts: 'dist/*.whl', fingerprint: true
    }
  }
}
